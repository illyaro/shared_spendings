package com.sharedSpendings;

import android.os.Bundle;
import android.util.Log;
import android.view.View;
import android.widget.TextView;
import android.widget.Toast;

import androidx.activity.EdgeToEdge;
import androidx.appcompat.app.AppCompatActivity;
import androidx.core.graphics.Insets;
import androidx.core.view.ViewCompat;
import androidx.core.view.WindowInsetsCompat;
import androidx.fragment.app.FragmentContainerView;
import androidx.recyclerview.widget.LinearLayoutManager;
import androidx.recyclerview.widget.RecyclerView;

import com.android.volley.AuthFailureError;
import com.android.volley.Request;
import com.android.volley.RequestQueue;
import com.android.volley.Response;
import com.android.volley.VolleyError;
import com.android.volley.toolbox.JsonObjectRequest;
import com.android.volley.toolbox.Volley;
import com.google.android.material.floatingactionbutton.FloatingActionButton;
import com.google.gson.Gson;
import com.sharedSpendings.newRecord.AddRecord;
import com.sharedSpendings.newRecord.NewRecord;
import com.sharedSpendings.newRecord.OnAddRecordInteractionListener;

import org.json.JSONObject;

import java.nio.charset.StandardCharsets;
import java.text.SimpleDateFormat;
import java.util.ArrayList;
import java.util.Date;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class UserMainScreen extends AppCompatActivity implements OnAddRecordInteractionListener {
    private RecordAdapter adapter;
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
        initializeRecycleView();
    }

    private void initializeRecycleView() {
        RecyclerView recyclerView = findViewById(R.id.records_recycler_view);
        recyclerView.setLayoutManager(new LinearLayoutManager(this));
        List<NewRecord> recordsToShow = fetchRecords(this.userID);
        this.adapter = new RecordAdapter(recordsToShow);
        recyclerView.setAdapter(this.adapter);
    }

    private List<NewRecord> fetchRecords(String userID) {
        return new ArrayList<>();
    }

    private void activateButtons() {
        FloatingActionButton btnAddRecord = findViewById(R.id.btn_add_record);
        FragmentContainerView fragmentAddRecord = findViewById(R.id.fragment_add_record);
        btnAddRecord.setOnClickListener(v -> {

            AddRecord addRecord = new AddRecord();
            getSupportFragmentManager()
                    .beginTransaction()
                    .replace(R.id.fragment_add_record, addRecord)
                    .commit();

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
    public void onConfirmSubmission(Double amount, Date datetime) {

        // TODO implement method adding to the database the data by calling corresponding endpoint.

        // convert datetime to iso 8601 string
        SimpleDateFormat sdf;
        sdf = new SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss");
        String dt = sdf.format(datetime);

        // creating object to send
        NewRecord record = new NewRecord(userID, amount, dt);
        Gson gson = new Gson();
        String json = gson.toJson(record);

        RequestQueue volleyQueue = Volley.newRequestQueue(UserMainScreen.this);
        String apiEndpoint = getResources().getString(R.string.uri_for_add_new_record);

        JsonObjectRequest jsonObjectRequest = new JsonObjectRequest(
                Request.Method.POST,
                apiEndpoint,
                null,
                new Response.Listener<JSONObject>() {
                    @Override
                    public void onResponse(JSONObject response) {
                        String responseObj = response.toString();
                        // TODO add inserted record to the recycler view.
                        NewRecord insertedRecord = gson.fromJson(responseObj, NewRecord.class);
                        adapter.addRecord(insertedRecord);
//                        Toast toast = Toast.makeText(UserMainScreen.this, String.format("Send request successfully to: %s, response: %s", apiEndpoint, insertedRecord.toString()), Toast.LENGTH_LONG);
//                        toast.show();
                    }
                },
                new Response.ErrorListener() {
                    @Override
                    public void onErrorResponse(VolleyError error) {
                        Toast toast = Toast.makeText(UserMainScreen.this, String.format("Error sending post request to: %s, error: %s", apiEndpoint, error.toString()), Toast.LENGTH_LONG);
                        toast.show();
                        Log.e("Request error", error.toString());
                    }
                }
        ) {
            @Override
            public byte[] getBody() {
                // Send the serialized object as the request body
                return json.getBytes(StandardCharsets.UTF_8);
            }

            @Override
            public Map<String, String> getHeaders() throws AuthFailureError {
                // Set the Authorization header
                Map<String, String> headers = new HashMap<>();
                headers.put("Authorization", "Bearer " + "my test request form android app");
                headers.put("Content-Type", "application/json");
                return headers;
            }
        };

        volleyQueue.add(jsonObjectRequest);

        this.onCloseAddRecord();
    }

}