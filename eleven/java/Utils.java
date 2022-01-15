package eleven;

import java.io.BufferedInputStream;
import java.io.IOException;
import java.util.Optional;

public class Utils {
    public static Optional<String> readStdin() {
        var bf = new BufferedInputStream(System.in);

        try {
            var input = bf.readAllBytes();
            return Optional.of(new String(input));
        } catch(Exception e) {
            return Optional.empty();
        }
    }

    public static void crash(String message) {
        System.out.println("[FATAL]");
        System.out.println(message);
        System.exit(1);
    }
}
