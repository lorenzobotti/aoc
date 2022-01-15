package eleven;

import java.util.ArrayList;

import eleven.SeaFloor;

public class Octopus {
    static final int energyForFlash = 9;
    int energy;
    boolean flashedThisTurn;

    Octopus(int energy) {
        this.energy = energy;
        this.flashedThisTurn = false;
    }

    Octopus(char energy) {
        this.energy = energy - '0';
        this.flashedThisTurn = false;
    }

    public void increment(SeaFloor floor, Coord at) {
        this.energy += 1;

        if (this.energy > energyForFlash) {
            this.flash(floor, at);
        }
    }

    public void newTurn() {
        this.flashedThisTurn = true;
    }

    void flash(SeaFloor floor, Coord at) {
        if (this.flashedThisTurn) {
            return;
        } else {
            this.flashedThisTurn = true;
        }
        
        this.energy = 0;
        ArrayList<Coord> neighbours = at.neighbours(floor.rows(), floor.columns());
        
        for (Coord neighbourPos : neighbours) {
            Octopus neighbour = floor.getOctopus(neighbourPos);
            neighbour.increment(floor, neighbourPos);
        }

        floor.countFlash();
    }

    public String toString() {
        return ""+this.energy;
    }
}