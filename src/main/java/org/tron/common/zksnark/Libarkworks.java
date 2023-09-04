package org.tron.common.zksnark;

class Libarkworks {
    private static final LibarkworksJNI INSTANCE = new LibarkworksJNI();
    private static final int FIELD_ELEMENT_SIZE = 32;

    public boolean libarkworksG1IsValid(byte[] x, byte[] y) {
        return INSTANCE.libarkworksG1IsValid(
            padLeft(x, FIELD_ELEMENT_SIZE), 
            padLeft(y, FIELD_ELEMENT_SIZE)
        );
    }
        
    public boolean libarkworksG2IsValid(byte[] a, byte[] b, byte[] c, byte[] d) {
        return INSTANCE.libarkworksG2IsValid(
            padLeft(a, FIELD_ELEMENT_SIZE), 
            padLeft(b, FIELD_ELEMENT_SIZE), 
            padLeft(c, FIELD_ELEMENT_SIZE), 
            padLeft(d, FIELD_ELEMENT_SIZE)
        );
    }
    
    public boolean libarkworksAddG1(byte[] a, byte[] b, byte[] result) {
        return INSTANCE.libarkworksAddG1(a, b, result);
    }
    
    public boolean libarkworksMulG1(byte[] p, byte[] s, byte[] result) {
        return INSTANCE.libarkworksMulG1(p, s, result);
    }
    
    public boolean libarkworksPairingCheck(byte[] g1s, byte[] g2s, int pairs) {
        return INSTANCE.libarkworksPairingCheck(g1s, g2s, pairs);
    }

    public void libarkworksRandomG1(byte[] g1) {
        INSTANCE.libarkworksRandomG1(g1);
    }

    public void libarkworksRandomG2(byte[] g2) {
        INSTANCE.libarkworksRandomG2(g2);
    }

    private static byte[] padLeft(byte[] array, int length) {
        if (array.length >= length) {
            return array;
        }
        byte[] result = new byte[length];
        System.arraycopy(array, 0, result, length - array.length, array.length);
        return result;
    }

    private static class LibarkworksJNI {        
        private native boolean libarkworksG1IsValid(byte[] x, byte[] y);
        
        private native boolean libarkworksG2IsValid(byte[] a, byte[] b, byte[] c, byte[] d);
        
        private native boolean libarkworksAddG1(byte[] a, byte[] b, byte[] result);
        
        private native boolean libarkworksMulG1(byte[] p, byte[] s, byte[] result);
        
        private native boolean libarkworksPairingCheck(byte[] g1s, byte[] g2s, int pairs);

        private native void libarkworksRandomG1(byte[] g1);

        private native void libarkworksRandomG2(byte[] g1);
    }
}
