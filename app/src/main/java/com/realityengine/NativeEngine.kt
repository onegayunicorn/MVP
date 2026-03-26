package com.realityengine

object NativeEngine {
    init {
        System.loadLibrary("reality_engine")
    }

    external fun processStatement(intent: Double, statement: String): Double
}
