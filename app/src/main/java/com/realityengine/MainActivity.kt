package com.realityengine

import android.content.Intent
import android.os.Bundle
import android.widget.Button
import android.widget.EditText
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        val statementInput = findViewById<EditText>(R.id.statementInput)
        val processButton = findViewById<Button>(R.id.processButton)
        val resultView = findViewById<TextView>(R.id.resultView)

        processButton.setOnClickListener {
            val statement = statementInput.text.toString()
            val intent = Intent(this, StatementService::class.java).apply {
                putExtra("statement", statement)
                putExtra("intent", 1.0) // Simplified intent value
            }
            startService(intent)
            resultView.text = "Processing statement..."
        }
    }
}
