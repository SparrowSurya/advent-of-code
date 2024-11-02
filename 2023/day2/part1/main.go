package main 

import (
	"os"
	"fmt"
	"log"
	"bufio"
	"strconv"
	"strings"
)

type Game struct {
	id uint
	cubes []Cubes
}

type Cubes struct {
	red uint
	green uint
	blue uint
}


func solve(games []Game) uint {
	var ans uint
	target := Cubes{12, 13, 14}

	for _, game := range games {
		var is_valid bool = true
		for _, cubes := range game.cubes {
			if cubes.red > target.red || cubes.green > target.green || cubes.blue > target.blue {
				is_valid = false
				break
			}
		}

		if is_valid {
			ans += game.id
		}
	}

	return ans
}

func parse(file *os.File) []Game {
	var games []Game
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		var slices []string = strings.Split(scanner.Text(), ": ")
		
		gid, _ := strconv.Atoi(slices[0][5:])

		var cubes []Cubes
		for _, rounds := range strings.Split(slices[1], "; ") {
			
			var r,g,b uint
			for _, dice := range strings.Split(rounds, ", ") {
				pairs := strings.Split(dice, " ") 
				count, _ := strconv.Atoi(pairs[0])
				switch pairs[1] {
					case "red":
						r += uint(count)
					case "green":
						g += uint(count)
					case "blue":
						b += uint(count)
				}
			}
			cubes = append(cubes, Cubes{r, g, b})
		}
		games = append(games, Game{uint(gid), cubes})
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	return games
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var games []Game = parse(file)

	ans := solve(games)
	fmt.Println(ans)
}
