package helpers

import (
	"bytes"
)

// AreInterfacesEqual checks if two interface are equal
func AreInterfacesEqual(first, second interface{}) bool {
	fb, err := GetInterfaceBytes(first)
	if err != nil {
		return false
	}
	sb, err := GetInterfaceBytes(second)
	if err != nil {
		return false
	}
	return bytes.Equal(fb, sb)
}