package eleven;

import java.util.ArrayList;

public class Coord {
    int row;
    int column;
    
    Coord(int row, int column) {
        this.row = row;
        this.column = column;
    }

    public ArrayList<Coord> neighbours(int rows, int columns) {
        var out = new ArrayList<Coord>();

        for (int row = Math.max(this.row-1, 0); row <= Math.min(this.row+1, rows-1); row++) {
            for (int column = Math.max(this.column-1, 0); column <= Math.min(this.column+1, columns-1); column++) {
                if (row == this.row && column == this.column) continue;
                
                out.add(new Coord(row, column));
            }
        }

        return out;
    }

    public int row() {
        return this.row;
    }
    public int column() {
        return this.column;
    }

    public String toString() {
        return "row: " + this.row + " column: " + this.column;
    }
    
}
