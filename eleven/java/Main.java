package eleven;

import eleven.Utils;
import eleven.SeaFloor;

import java.util.ArrayList;

import eleven.Coord;

public class Main {
    public static void main(String[] Args) {
        // Coord coord = new Coord(10, 10);
        // ArrayList<Coord> neighbours = coord.neighbours(11, 12);
        // for(Coord c : neighbours) {
        //     System.out.println(c);
        // }
        // System.out.println(neighbours.size());




        String input = Utils.readStdin().orElseThrow();

        SeaFloor floor = new SeaFloor(input);
        for (int i = 0; i < 10; i++) {
            System.out.println(floor.toString());
            floor.turn();
        }
    }
}