package org.tron.common.zksnark;

import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertTrue;

import java.util.Arrays;

import org.junit.Test;

public class LibarkworksTest {
    @Test
    public void libarkworksG1IsValid() {
        byte[] g1 = new byte[64];
        byte[][] g1XY = getG1XY(g1);
        boolean isValid = LibarkworksWrapper.getInstance().libarkworksG1IsValid(g1XY[0], g1XY[1]);
        assertTrue("G1 point should be valid", isValid);

        isValid = LibarkworksWrapper.getInstance().libarkworksG1IsValid(new byte[0], new byte[0]);
        assertTrue("G1 point should be valid", isValid);

        LibarkworksWrapper.getInstance().libarkworksRandomG1(g1);
        g1XY = getG1XY(g1);
        isValid = LibarkworksWrapper.getInstance().libarkworksG1IsValid(g1XY[0], g1XY[1]);
        assertTrue("G1 point should be valid", isValid);

        isValid = LibarkworksWrapper.getInstance().libarkworksG1IsValid(new byte[32], g1XY[1]);
        assertFalse("G1 point should not be valid", isValid);
    }

    @Test
    public void libarkworksG2IsValid() {
        byte[] g2 = new byte[128];
        byte[][] g2ABCD = getG2ABCD(g2);
        boolean isValid = LibarkworksWrapper.getInstance().libarkworksG2IsValid(g2ABCD[0], g2ABCD[1], g2ABCD[2],
                g2ABCD[3]);
        assertTrue("G2 point should be valid", isValid);

        isValid = LibarkworksWrapper.getInstance().libarkworksG2IsValid(new byte[0], new byte[0], new byte[0],
                new byte[0]);
        assertTrue("G2 point should be valid", isValid);

        LibarkworksWrapper.getInstance().libarkworksRandomG2(g2);
        g2ABCD = getG2ABCD(g2);
        isValid = LibarkworksWrapper.getInstance().libarkworksG2IsValid(g2ABCD[0], g2ABCD[1], g2ABCD[2], g2ABCD[3]);
        assertTrue("G1 point should be valid", isValid);

        isValid = LibarkworksWrapper.getInstance().libarkworksG2IsValid(new byte[32], g2ABCD[1], g2ABCD[2], g2ABCD[3]);
        assertFalse("G1 point should not be valid", isValid);
    }

    @Test
    public void libarkworksAddG1() {
        byte[] a = new byte[64];
        LibarkworksWrapper.getInstance().libarkworksRandomG1(a);
        printHex(a);

        byte[] b = new byte[64];
        LibarkworksWrapper.getInstance().libarkworksRandomG1(b);
        printHex(b);

        byte[] result = new byte[64];
        boolean success = LibarkworksWrapper.getInstance().libarkworksAddG1(a, b, result);
        assertTrue("Addition should be successful", success);
        printHex(result);
    }

    @Test
    public void libarkworksMulG1() {
        byte[] p = new byte[64];
        LibarkworksWrapper.getInstance().libarkworksRandomG1(p);
        printHex(p);

        byte[] s = new byte[32];
        s[0] = 5;
        printHex(s);

        byte[] result = new byte[64];
        boolean success = LibarkworksWrapper.getInstance().libarkworksMulG1(p, s, result);
        assertTrue("Multiplication should be successful", success);
        printHex(result);
    }

    @Test
    public void libarkworksPairingCheck() {
        int pairs = 4;
        byte[] g1s = new byte[0];
        byte[] g2s = new byte[0];

        for (int i = 0; i < pairs; i++) {
            byte[] g1 = new byte[64];
            LibarkworksWrapper.getInstance().libarkworksRandomG1(g1);
            g1s = concat(g1s, g1);
            
            byte[] g2 = new byte[128];
            LibarkworksWrapper.getInstance().libarkworksRandomG2(g2);
            g2s = concat(g2s, g2);
        }

        boolean success = LibarkworksWrapper.getInstance().libarkworksPairingCheck(g1s, g2s, pairs);
        assertFalse("Pairing check should NOT be successful", success);
    }

    private static byte[][] getG1XY(byte[] g1) {
        byte[] x = Arrays.copyOfRange(g1, 0, 32);
        byte[] y = Arrays.copyOfRange(g1, 32, 64);
        return new byte[][] { x, y };
    }

    private static byte[][] getG2ABCD(byte[] g2) {
        byte[] a = Arrays.copyOfRange(g2, 0, 32);
        byte[] b = Arrays.copyOfRange(g2, 32, 64);
        byte[] c = Arrays.copyOfRange(g2, 64, 96);
        byte[] d = Arrays.copyOfRange(g2, 96, 128);
        return new byte[][] { a, b, c, d };
    }

    static byte[] concat(byte[] g1s, byte[] g1) {
        byte[] result = Arrays.copyOf(g1s, g1s.length + g1.length);
        System.arraycopy(g1, 0, result, g1s.length, g1.length);
        return result;
    }

    private static void printHex(byte[] byteArray) {
        String hex = "";

        // Iterating through each byte in the array
        for (byte i : byteArray) {
            hex += String.format("%02X", i);
        }

        System.out.println(hex);
    }
}
