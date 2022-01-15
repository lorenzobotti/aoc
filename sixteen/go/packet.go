package main

type Packet struct {
	version byte
	typeID  byte

	literalValue int
	subPackets   []Packet
}

func (p Packet) VersionSum() int {
	out := int(p.version)

	for _, child := range p.subPackets {
		out += child.VersionSum()
	}

	return out
}

func (p Packet) IsOperator() bool {
	return p.typeID != 4
}

func (p Packet) IsLiteral() bool {
	return p.typeID == 4
}
