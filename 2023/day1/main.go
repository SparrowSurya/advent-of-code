package main

import (
	"fmt"
	"bufio"
	"log"
	"os"
)

func calibrationValue(text string) uint {
	var first, last rune
	for _, ch := range text {
		if '0' <= ch && ch <= '9' {
			if first == '\u0000' {
				first = ch
			} else {
				last = ch
			}
		}
	}

	if last == '\u0000' {
		last = first
	}
	
	return uint(first - '0') * 10 + uint(last - '0')
}

func solve(lines []string) uint {
	var count uint
	
	for _, line := range lines {
		count += calibrationValue(line)
	}
	
	return count
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	ans := solve(lines)
	fmt.Println(ans)
}
