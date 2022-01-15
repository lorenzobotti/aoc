package eleven;

import eleven.Coord;
import eleven.Octopus;
import java.util.ArrayList;

public class SeaFloor {
    ArrayList<ArrayList<Octopus>> dumbos;
    int flashes;

    SeaFloor(String input) {
        var lines = input.split("\n", -1);
        this.dumbos = new ArrayList<ArrayList<Octopus>>();
        this.flashes = 0;

        for (var line : lines) {
            var row = new ArrayList<Octopus>();
            for (char ch : line.toCharArray()) {
                row.add(new Octopus(ch));
            }
            this.dumbos.add(row);
        }
    }

    public int rows() {
        return this.dumbos.size();
    }
    
    public int columns() {
        return this.dumbos.get(0).size();
    }

    public Octopus getOctopus(Coord c) {
        int row = c.row();
        int column = c.column();
            
        return this.dumbos.get(row).get(column);
    }
    
    public void setOctopus(Octopus oct, Coord c) {
        int row = c.row();
        int column = c.column();

        this.dumbos.get(row).set(column, oct);
    }

    public void countFlash() {
        this.flashes += 1;
    }

    public void turn() {
        for (ArrayList<Octopus> row : this.dumbos) {
            for (Octopus oct : row) {
                oct.newTurn();
            }
        }
        
        for (int row = 0; row < this.rows(); row++) {
            for (int column = 0; column < this.columns(); column++) {
                var c = new Coord(row, column);
                this.dumbos.get(row).get(column).increment(this, c);
            }
        }
    }


    public String toString() {
        String output = "";

        for (ArrayList<Octopus> row : this.dumbos) {
            for (Octopus oct : row) {
                output += oct.toString();
            }
            output += "\n";
        }

        return output;
    }
        
}