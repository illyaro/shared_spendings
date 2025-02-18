package com.sharedSpendings;

import android.os.Bundle;
import android.widget.TextView;

import androidx.activity.EdgeToEdge;
import androidx.appcompat.app.AppCompatActivity;
import androidx.core.graphics.Insets;
import androidx.core.view.ViewCompat;
import androidx.core.view.WindowInsetsCompat;

public class UserMainScreen extends AppCompatActivity {
    private String userID;
    private String email;
    private String userPhoto;
    private String username;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        EdgeToEdge.enable(this);
        setContentView(R.layout.activity_user_main_screen);
        ViewCompat.setOnApplyWindowInsetsListener(findViewById(R.id.main), (v, insets) -> {
            Insets systemBars = insets.getInsets(WindowInsetsCompat.Type.systemBars());
            v.setPadding(systemBars.left, systemBars.top, systemBars.right, systemBars.bottom);
            return insets;
        });

        extractParameters();
    }

    private void extractParameters() {
        userID = getIntent().getStringExtra("userID");
        email = getIntent().getStringExtra("email");
        userPhoto = getIntent().getStringExtra("userPhoto");
        username = getIntent().getStringExtra("username");

        TextView tv1 = findViewById(R.id.text1);
        tv1.setText(userID);

        TextView tv2 = findViewById(R.id.text2);
        tv2.setText(email);

        TextView tv3 = findViewById(R.id.text3);
        tv3.setText(userPhoto);

        TextView tv4 = findViewById(R.id.text4);
        tv4.setText(username);
    }
}