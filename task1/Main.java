import java.io.*;
import java.util.*;

public class Main {
    static boolean isPrime(int n) {
        if (n < 2) return false;
        if (n == 2) return true;
        if (n % 2 == 0) return false;
        for (int i = 3; i * i <= n; i += 2) {
            if (n % i == 0) return false;
        }
        return true;
    }

    static int nearestPrimeDistance(int n) {
        if (isPrime(n)) return 0;
        int d = 1;
        while (true) {
            if (isPrime(n - d) || isPrime(n + d)) return d;
            d++;
        }
    }

    public static void main(String[] args) throws Exception {
        int N;
        File file = new File("input.txt");

        if (file.exists()) {
            Scanner fin = new Scanner(file);
            N = fin.nextInt();
            fin.close();
        } else {
            Scanner in = new Scanner(System.in);
            System.out.print("Введите N: ");
            N = in.nextInt();
        }

        int result = nearestPrimeDistance(N);

        System.out.println("Результат: " + result);

        PrintWriter fout = new PrintWriter("output.txt");
        fout.println(result);
        fout.close();
    }
}
