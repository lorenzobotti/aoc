package main

func Play(players [2]Player) int {
	die := NewDie()

	loser := 0

outer:
	for {
		// pp.Println(players)
		for i, player := range players {
			player.Move(&die)
			if player.Won() {
				loser = 1 - i
				break outer
			}
		}
	}

	loserScore := players[loser].Score

	solution := loserScore * die.timesRolled
	return solution
}
