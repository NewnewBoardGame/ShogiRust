package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	fmt.Println("Hello, world! from Go!")

	fmt.Println("Command line arguments given: ", os.Args)

	var file *os.File
	var err error
	if len(os.Args) < 2 {
		file, err = os.Open(os.Args[1])
	}
	if err != nil {
		file, err = os.Open("input.txt")
	}

	if err != nil {
		fmt.Println("Unable to read from a file, exiting..")
		os.Exit(1)
	}

	calculateNoOfValidStrings(file)
}

func calculateNoOfValidStrings(file *os.File) int {
	var noOfValidStrings = 0
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		if stringHasRepetition(line) {
			noOfValidStrings++
		}
	}
	fmt.Println("Calculated of valid Strings: ", noOfValidStrings)
	return noOfValidStrings
}

func stringHasRepetition(s string) bool {
	for i := 0; i < len(s); i++ {
		for j := i + 1; j < len(s); j++ {
			if s[i] == s[j] {
				diff := j - i
				for k := 1; k < diff; k++ {
					if s[i+k] != s[j+k] {
						return false
					}
				}
			}
		}
	}
	return true
}
