package main

func (b *BoolReader) ParsePacket() Packet {
	out := Packet{}

	out.version = byte(toNumber(b.Consume(3)))
	out.typeID = byte(toNumber(b.Consume(3)))

	if out.typeID == 4 {
		out.literalValue = b.ParseLiteralSection()
	} else {
		lengthTypeId := b.Consume(1)[0]

		if !lengthTypeId {
			// the next 15 bits are a number that
			// represents the total length in bits
			//  of the sub-packets contained by this packet.

			lengthOfChildren := toNumber(b.Consume(15))
			childrenReader := newBoolReader(b.Consume(lengthOfChildren))

			for !childrenReader.EOF() {
				out.subPackets = append(out.subPackets, childrenReader.ParsePacket())
			}

		} else {
			// the next 11 bits are a number that
			// represents the number of sub-packets
			// immediately contained by this packet.

			subPackets := toNumber(b.Consume(11))
			for i := 0; i < subPackets; i++ {
				out.subPackets = append(out.subPackets, b.ParsePacket())
			}
		}
	}

	return out
}

func (b *BoolReader) ParseLiteralSection() int {
	boolOut := []bool{}
	for {
		fiveBits := b.Consume(5)
		keepReading := fiveBits[0]

		boolOut = append(boolOut, fiveBits[1:]...)

		if !keepReading {
			break
		}
	}

	return toNumber(boolOut)
}
