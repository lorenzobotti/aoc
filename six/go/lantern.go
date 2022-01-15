package main

import (
	"bytes"
	"fmt"
	"strconv"
	"strings"
)

const lanternCycle = 7
const lanternPuberty = 2

type Lantern struct {
	daysUntilKid int
	sea          *Sea
}

func NewLantern(days int, sea *Sea) *Lantern {
	return &Lantern{daysUntilKid: days, sea: sea}
}

func (l *Lantern) Tick() *Lantern {
	if l.daysUntilKid == 0 {
		kid := NewLantern(
			lanternCycle+lanternPuberty-1,
			l.sea,
		)

		l.daysUntilKid = lanternCycle - 1
		return kid
	} else {
		l.daysUntilKid -= 1
	}

	return nil
}

type Sea struct {
	lanterns []*Lantern
}

func (s *Sea) AddLantern(l *Lantern) {
	s.lanterns = append(s.lanterns, l)
}

func ParseSea(input string) (Sea, error) {
	input = strings.TrimSpace(input)
	nums := strings.Split(input, ",")

	sea := Sea{}
	for _, numRaw := range nums {
		num, err := strconv.Atoi(numRaw)
		if err != nil {
			return Sea{}, err
		}

		sea.lanterns = append(sea.lanterns, NewLantern(num, &sea))
	}

	return sea, nil
}

func (s *Sea) Tick() {
	for _, lantern := range s.lanterns {
		child := lantern.Tick()
		if child != nil {
			s.lanterns = append(s.lanterns, child)
		}
	}
}

func (s Sea) Population() int {
	return len(s.lanterns)
}

func (s Sea) String() string {
	builder := bytes.Buffer{}

	for _, lantern := range s.lanterns {
		fmt.Fprintf(&builder, "%d, ", lantern.daysUntilKid)
	}

	return builder.String()
}
