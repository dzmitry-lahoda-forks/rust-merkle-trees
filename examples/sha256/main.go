package main

import (
	"fmt"

	"github.com/ComposableFi/go-merkle-trees/merkle"
)

func main() {
	leaves := []string{
		"Hello",
		"Dorood",
		"Hi",
		"Hey",
		"Hola",
	}
	var leavesI [][]byte
	for _, l := range leaves {
		h, _ := Sha256Hasher{}.Hash([]byte(l))
		leavesI = append(leavesI, h)
	}

	merkleTree := merkle.NewTree(Sha256Hasher{})
	merkleTree, err := merkleTree.FromLeaves(leavesI)
	if err != nil {
		panic(err)
	}

	root := merkleTree.Root()
	fmt.Printf("Merkle root is %v \n", merkleTree.RootHex())

	// build merkle proof for 42 (its index is 1);
	proof := merkleTree.Proof([]uint32{1})

	fmt.Printf("merkle proof hashes are:\n")
	for _, v := range proof.ProofHashesHex() {
		fmt.Printf(" - %v\n", v)
	}

	// verify merkle proof
	verifyResult, err := proof.Verify(root)
	if err != nil {
		panic(err)
	} else if !verifyResult {
		panic("merkle proof verify result is false")
	}
	fmt.Printf("merkle proof verify result is %v\n", verifyResult)

}
