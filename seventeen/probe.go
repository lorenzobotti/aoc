package main

type Probe struct {
	pos    Coord
	xSpeed int
	ySpeed int
}

func (p *Probe) Move() {
	p.pos.x += p.xSpeed
	p.pos.y += p.ySpeed

	if p.xSpeed > 0 {
		p.xSpeed -= 1
	} else if p.xSpeed < 0 {
		p.xSpeed += 1
	}

	p.ySpeed -= 1
}

func (p Probe) Hits(target Area) (bool, int) {
	highest := p.pos.y
	for p.MightHit(target) {
		p.Move()

		if p.pos.y > highest {
			highest = p.pos.y
		}

		if target.Hit(p.pos) {
			return true, highest
		}
	}

	return false, 0
}

func (p Probe) MightHit(target Area) bool {
	goingDown := p.ySpeed <= 0
	belowTarget := p.pos.y < target.yBottom

	// assuming we're never going left
	pastTarget := p.pos.x > target.xRight

	return !((goingDown && belowTarget) || pastTarget)
}
