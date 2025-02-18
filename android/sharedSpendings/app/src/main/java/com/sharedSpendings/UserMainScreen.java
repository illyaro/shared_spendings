package com.sharedSpendings;

import android.os.Bundle;
import android.view.View;
import android.widget.TextView;
import android.widget.Toast;

import androidx.activity.EdgeToEdge;
import androidx.appcompat.app.AppCompatActivity;
import androidx.core.graphics.Insets;
import androidx.core.view.ViewCompat;
import androidx.core.view.WindowInsetsCompat;
import androidx.fragment.app.FragmentContainerView;

import com.google.android.material.floatingactionbutton.FloatingActionButton;

public class UserMainScreen extends AppCompatActivity implements OnAddRecordInteractionListener {
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
        activateButtons();
    }

    private void activateButtons() {
        FloatingActionButton btnAddRecord = findViewById(R.id.btn_add_record);
        FragmentContainerView fragmentAddRecord = findViewById(R.id.fragment_add_record);
        btnAddRecord.setOnClickListener(v -> {
            fragmentAddRecord.setVisibility(View.VISIBLE);
            btnAddRecord.setVisibility(View.INVISIBLE);
        });
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

    // Implements method of the interface OnAddRecordInteractionListener.
    @Override
    public void onCloseAddRecord() {
        FragmentContainerView fragmentAddRecord = findViewById(R.id.fragment_add_record);
        FloatingActionButton btnAddRecord = findViewById(R.id.btn_add_record);
        fragmentAddRecord.setVisibility(View.INVISIBLE);
        btnAddRecord.setVisibility(View.VISIBLE);
    }

    // Implements method of the interface OnAddRecordInteractionListener.
    @Override
    public void onConfirmSubmission(String amount, String datetime) {
        Toast toast = Toast.makeText(this, String.format("Amount: %s,\t date: %s", amount, datetime), Toast.LENGTH_LONG);
        toast.show();
        // TODO implement method adding to the database the data by calling corresponding endpoint.

        this.onCloseAddRecord();
    }

}