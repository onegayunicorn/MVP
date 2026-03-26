package com.realityengine

import android.app.Service
import android.content.Intent
import android.os.IBinder
import android.util.Log

class StatementService : Service() {
    private val TAG = "StatementService"

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        val statement = intent?.getStringExtra("statement") ?: ""
        val intentValue = intent?.getDoubleExtra("intent", 0.0) ?: 0.0

        Log.d(TAG, "Processing statement: $statement")

        try {
            val result = NativeEngine.processStatement(intentValue, statement)
            Log.d(TAG, "Reality calculation result: $result")
            // Handle result (e.g., broadcast to UI or update state)
        } catch (e: Exception) {
            Log.e(TAG, "Error processing statement", e)
        }

        return START_NOT_STICKY
    }

    override fun onBind(intent: Intent?): IBinder? = null
}
