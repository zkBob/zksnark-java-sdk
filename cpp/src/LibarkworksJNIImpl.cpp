//
// Created by ruipeng on 6/5/19.
//

#include "org_tron_common_zksnark_Libarkworks_LibarkworksJNI.h"
#include "libarkworks.h"
#include <iostream>

jboolean bool2jboolean(bool b) {
    if (b) {
        return JNI_TRUE;
    } else {
        return JNI_FALSE;
    }
}

/*
 * Class:     org_tron_common_zksnark_Libarkworks_LibarkworksJNI
 * Method:    libarkworksG1IsValid
 * Signature: ([B[B)Z
 */
JNIEXPORT jboolean JNICALL Java_org_tron_common_zksnark_Libarkworks_00024LibarkworksJNI_libarkworksG1IsValid
  (JNIEnv * env, jobject, jbyteArray xBytes, jbyteArray yBytes) {
    unsigned char * x = (unsigned char *) env-> GetByteArrayElements(xBytes, nullptr);
    unsigned char * y = (unsigned char *) env-> GetByteArrayElements(yBytes, nullptr);
    if (x == NULL || y == NULL) {
      return JNI_FALSE;
    }
    bool result = libarkworks_g1_is_valid(x, y);
    env->ReleaseByteArrayElements(xBytes,(jbyte*)x,0);
    env->ReleaseByteArrayElements(yBytes,(jbyte*)y,0);
    return bool2jboolean(result);
}

/*
 * Class:     org_tron_common_zksnark_Libarkworks_LibarkworksJNI
 * Method:    libarkworksG2IsValid
 * Signature: ([B[B[B[B)Z
 */
JNIEXPORT jboolean JNICALL Java_org_tron_common_zksnark_Libarkworks_00024LibarkworksJNI_libarkworksG2IsValid
  (JNIEnv * env, jobject, jbyteArray aBytes, jbyteArray bBytes, jbyteArray cBytes, jbyteArray dBytes) {
    unsigned char * a = (unsigned char *) env-> GetByteArrayElements(aBytes, nullptr);
    unsigned char * b = (unsigned char *) env-> GetByteArrayElements(bBytes, nullptr);
    unsigned char * c = (unsigned char *) env-> GetByteArrayElements(cBytes, nullptr);
    unsigned char * d = (unsigned char *) env-> GetByteArrayElements(dBytes, nullptr);
    if (a == NULL || b == NULL || c == NULL || d == NULL) {
      return JNI_FALSE;
    }
    bool result = libarkworks_g2_is_valid(a, b, c, d);
    env->ReleaseByteArrayElements(aBytes,(jbyte*)a,0);
    env->ReleaseByteArrayElements(bBytes,(jbyte*)b,0);
    env->ReleaseByteArrayElements(cBytes,(jbyte*)c,0);
    env->ReleaseByteArrayElements(dBytes,(jbyte*)d,0);
    return bool2jboolean(result);
}

/*
 * Class:     org_tron_common_zksnark_Libarkworks_LibarkworksJNI
 * Method:    libarkworksAddG1
 * Signature: ([B[B[B)Z
 */
JNIEXPORT jboolean JNICALL Java_org_tron_common_zksnark_Libarkworks_00024LibarkworksJNI_libarkworksAddG1
  (JNIEnv * env, jobject, jbyteArray aBytes, jbyteArray bBytes, jbyteArray result) {
    unsigned char * a = (unsigned char *) env-> GetByteArrayElements(aBytes, nullptr);
    unsigned char * b = (unsigned char *) env-> GetByteArrayElements(bBytes, nullptr);
    unsigned char * r = (unsigned char *) env-> GetByteArrayElements(result, nullptr);
    if (a == NULL || b == NULL || r == NULL) {
      return JNI_FALSE;
    }
    bool success = libarkworks_add_g1(a, b, r);
    env->ReleaseByteArrayElements(aBytes,(jbyte*)a,0);
    env->ReleaseByteArrayElements(bBytes,(jbyte*)b,0);
    env->ReleaseByteArrayElements(result,(jbyte*)r,0);
    return bool2jboolean(success);
}

/*
 * Class:     org_tron_common_zksnark_Libarkworks_LibarkworksJNI
 * Method:    libarkworksMulG1
 * Signature: ([B[B[B)Z
 */
JNIEXPORT jboolean JNICALL Java_org_tron_common_zksnark_Libarkworks_00024LibarkworksJNI_libarkworksMulG1
  (JNIEnv * env, jobject, jbyteArray pBytes, jbyteArray sBytes, jbyteArray result) {
    unsigned char * p = (unsigned char *) env-> GetByteArrayElements(pBytes, nullptr);
    unsigned char * s = (unsigned char *) env-> GetByteArrayElements(sBytes, nullptr);
    unsigned char * r = (unsigned char *) env-> GetByteArrayElements(result, nullptr);
    if (p == NULL || s == NULL || r == NULL) {
      return JNI_FALSE;
    }
    bool success = libarkworks_mul_g1(p, s, r);
    env->ReleaseByteArrayElements(pBytes,(jbyte*)p,0);
    env->ReleaseByteArrayElements(sBytes,(jbyte*)s,0);
    env->ReleaseByteArrayElements(result,(jbyte*)r,0);
    return bool2jboolean(success);
}

/*
 * Class:     org_tron_common_zksnark_Libarkworks_LibarkworksJNI
 * Method:    libarkworksPairingCheck
 * Signature: ([B[BI)Z
 */
JNIEXPORT jboolean JNICALL Java_org_tron_common_zksnark_Libarkworks_00024LibarkworksJNI_libarkworksPairingCheck
  (JNIEnv * env, jobject, jbyteArray g1sBytes, jbyteArray g2sBytes, jint pairs) {
    unsigned char * g1s = (unsigned char *) env-> GetByteArrayElements(g1sBytes, nullptr);
    unsigned char * g2s = (unsigned char *) env-> GetByteArrayElements(g2sBytes, nullptr);
    if (g1s == NULL || g2s == NULL) {
      return JNI_FALSE;
    }
    bool success = libarkworks_pairing_check(g1s, g2s, pairs);
    env->ReleaseByteArrayElements(g1sBytes,(jbyte*)g1s,0);
    env->ReleaseByteArrayElements(g2sBytes,(jbyte*)g2s,0);
    return bool2jboolean(success);
}

/*
 * Class:     org_tron_common_zksnark_Libarkworks_LibarkworksJNI
 * Method:    libarkworksRandomG1
 * Signature: ([B)V
 */
JNIEXPORT void JNICALL Java_org_tron_common_zksnark_Libarkworks_00024LibarkworksJNI_libarkworksRandomG1
    (JNIEnv * env, jobject, jbyteArray g1) {
    unsigned char * r = (unsigned char *) env-> GetByteArrayElements(g1, nullptr);
    if (r == NULL) {
      return;
    }
    libarkworks_random_g1(r);
    env->ReleaseByteArrayElements(g1,(jbyte*)r,0);
}

/*
 * Class:     org_tron_common_zksnark_Libarkworks_LibarkworksJNI
 * Method:    libarkworksRandomG2
 * Signature: ([B)V
 */
JNIEXPORT void JNICALL Java_org_tron_common_zksnark_Libarkworks_00024LibarkworksJNI_libarkworksRandomG2
  (JNIEnv * env, jobject, jbyteArray g2) {
    unsigned char * r = (unsigned char *) env-> GetByteArrayElements(g2, nullptr);
    if (r == NULL) {
      return;
    }
    libarkworks_random_g2(r);
    env->ReleaseByteArrayElements(g2,(jbyte*)r,0);
}