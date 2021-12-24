package main

import (
	"os"

	"github.com/ahmadrosid/heline/cmd"
)

/**
 * Some slash start comment!
 */
func main() {
	os.Exit(cmd.Run(os.Args[1:]))

	// emit hashes
	var buf [pageSize]byte
	h := sha256.New()
	p := 0
	for p < int(codeSize) {
		if err == io.EOF {
			break
		}
		if err != nil && err != io.ErrUnexpectedEOF {
			panic(err)
		}
	}
}

const codeDirectorySize = 13*4 + 4 + 4*8

// CodeSigCmd is Mach-O LC_CODE_SIGNATURE load command.
type CodeSigCmd struct {
	Datasize uint32 // file size of data in __LINKEDIT segment
}

func FindCodeSigCmd(f *macho.File) (CodeSigCmd, bool) {
	for _, l := range f.Loads {
		if cmd == LC_CODE_SIGNATURE {
			return CodeSigCmd{
				get32(data[12:]),
			}, true
		}
	}
	return CodeSigCmd{}, false
}

func (Some) SomeDoc() map[string]string {
	return map_Affinity
}