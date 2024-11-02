package main 

import (
	"os"
	"fmt"
	"log"
	"bufio"
)

type Engine struct {
	schema []string
	width uint
	height uint
}

func isSymbol(c byte) bool {
	return (c < '0' || c > '9') && (c != '.')
}

func inRange(num, low, high int) bool {
    return num >= low && num < high
}

func nearSymbol(e Engine, r, c uint) bool {
	dirs := [][2]int{{0, 1}, {1, 1}, {1, 0}, {1, -1}, {0, -1}, {-1, -1}, {-1, 0}, {-1, 1}}

	for _, d := range dirs {
		var nr int = int(r) + d[0]
		var nc int = int(c) + d[1]
		if inRange(nr, 0, int(e.height)) && inRange(nc, 0, int(e.width)) && isSymbol(e.schema[nr][nc]) {
			return true
		}
	}

	return false
}

func findNum(e Engine, r, c uint) (uint, uint) {
	var num uint
	var length uint
	
	for i := c; i < e.width; i++ {
		ch := e.schema[r][i]
		if !('0' <= ch && ch <= '9') {
			 break
		}

		num *= 10
		num += uint(ch - '0')
		length += 1
	}

	for j := c; j < (c + length); j++ {
		if nearSymbol(e, r, j) {
			return num, length
		}
	}

	return 0, 0
}

func solve(e Engine) uint {
	var total uint

	var r uint
	for ; r < e.height; r++ {
		var c uint
		for ; c < e.width; {
			ch := e.schema[r][c]
			if '0' <= ch && ch <= '9' {
				num, length := findNum(e, r, c)
				if length > 0 {
					total += num
					c += length
					continue
				}
			}
			c += 1
		}
	}

	return total
}

func parse(file *os.File) Engine {
	scanner := bufio.NewScanner(file)
	var strs []string
	for scanner.Scan() {
		strs = append(strs, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	
	height := len(strs)
	width := len(strs[0])
	return Engine{strs, uint(width), uint(height)}
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var engine Engine = parse(file)
	ans := solve(engine)
	fmt.Println(ans)
}
