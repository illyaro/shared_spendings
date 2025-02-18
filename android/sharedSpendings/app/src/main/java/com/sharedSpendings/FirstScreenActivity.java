package com.sharedSpendings;

import android.content.Intent;
import android.os.Bundle;
import android.widget.Button;


import androidx.activity.EdgeToEdge;
import androidx.appcompat.app.AppCompatActivity;
import androidx.core.graphics.Insets;
import androidx.core.view.ViewCompat;
import androidx.core.view.WindowInsetsCompat;

public class FirstScreenActivity extends AppCompatActivity {

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        EdgeToEdge.enable(this);
        setContentView(R.layout.activity_first_screen);
        ViewCompat.setOnApplyWindowInsetsListener(findViewById(R.id.main), (v, insets) -> {
            Insets systemBars = insets.getInsets(WindowInsetsCompat.Type.systemBars());
            v.setPadding(systemBars.left, systemBars.top, systemBars.right, systemBars.bottom);
            return insets;
        });
        
        activateButtons();
    }

    private void activateButtons() {
        Button enterButton = findViewById(R.id.login_button);
        Intent goToMainUserScreen = new Intent();
        // add userID and other parameters to the intent
        goToMainUserScreen.putExtra("userID", "105273289810024784473");
        goToMainUserScreen.putExtra("email", "panvalkon@gmail.com");
        goToMainUserScreen.putExtra("userPhoto", "https://lh3.googleusercontent.com/a/ACg8ocI-DpFFouaj-OcsZop_h9njgWnMJIAs96Ts78zXpoyjcptX5zmK=s96-c");
        goToMainUserScreen.putExtra("username", "ILLYA");
        goToMainUserScreen.setClass(this, UserMainScreen.class);
        enterButton.setOnClickListener(v -> startActivity(goToMainUserScreen));
    }
}