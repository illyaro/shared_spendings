package com.sharedSpendings.newRecord;

import android.content.Context;
import android.os.Bundle;

import androidx.annotation.NonNull;
import androidx.constraintlayout.widget.ConstraintLayout;
import androidx.fragment.app.Fragment;
import androidx.fragment.app.FragmentContainerView;

import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.DatePicker;
import android.widget.EditText;
import android.widget.TextView;

import com.google.android.material.floatingactionbutton.FloatingActionButton;
import com.sharedSpendings.R;

import java.text.DateFormat;
import java.util.Calendar;
import java.util.Date;
import java.util.Locale;

/**
 * A simple {@link Fragment} subclass.
 * Use the {@link AddRecord#newInstance} factory method to
 * create an instance of this fragment.
 */
public class AddRecord extends Fragment {

    private OnAddRecordInteractionListener interactionListener;

    // TODO: Rename parameter arguments, choose names that match
    // the fragment initialization parameters, e.g. ARG_ITEM_NUMBER
    private static final String ARG_PARAM1 = "param1";
    private static final String ARG_PARAM2 = "param2";

    // TODO: Rename and change types of parameters
    private String mParam1;
    private String mParam2;

    public AddRecord() {
        // Required empty public constructor
    }

    /**
     * Use this factory method to create a new instance of
     * this fragment using the provided parameters.
     *
     * @param param1 Parameter 1.
     * @param param2 Parameter 2.
     * @return A new instance of fragment AddRecord.
     */
    // TODO: Rename and change types and number of parameters
    public static AddRecord newInstance(String param1, String param2) {
        AddRecord fragment = new AddRecord();
        Bundle args = new Bundle();
        args.putString(ARG_PARAM1, param1);
        args.putString(ARG_PARAM2, param2);
        fragment.setArguments(args);
        return fragment;
    }

    @Override
    public void onAttach(@NonNull Context context) {
        super.onAttach(context);
        if (context instanceof OnAddRecordInteractionListener) {
            interactionListener = (OnAddRecordInteractionListener) context;
        } else {
            throw new RuntimeException(context.toString() + " must implement OnAddRecordInteractionListener");
        }
    }

    @Override
    public void onDetach() {
        super.onDetach();
        interactionListener = null;
    }

    @Override
    public void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        if (getArguments() != null) {
            mParam1 = getArguments().getString(ARG_PARAM1);
            mParam2 = getArguments().getString(ARG_PARAM2);
        }
    }

    @Override
    public View onCreateView(LayoutInflater inflater, ViewGroup container,
                             Bundle savedInstanceState) {
        // Inflate the layout for this fragment
        return inflater.inflate(R.layout.fragment_add_record, container, false);
    }

    @Override
    public void onViewCreated(@NonNull View view, Bundle savedInstanceState){
        super.onViewCreated(view, savedInstanceState);

        // Set default date to current.
        TextView dateTime = view.findViewById(R.id.purchase_date_text_field);
        Date currentDate = new Date();
        DateFormat df = DateFormat.getDateInstance(DateFormat.DEFAULT, Locale.getDefault());
        String daterFormatted = df.format(currentDate);
        dateTime.setText(daterFormatted);

        // Close button handler
        FloatingActionButton btnCloseAddRecord = view.findViewById(R.id.add_new_record_close_button);
        btnCloseAddRecord.setOnClickListener(v -> {
            if (interactionListener != null) {
                interactionListener.onCloseAddRecord();
            }
        });

        // Calendar button
        FloatingActionButton btnSetPurchaseDate = view.findViewById(R.id.btn_set_purchase_date);
        ConstraintLayout fragmentDatePicker = view.findViewById(R.id.fragment_data_picker);
        btnSetPurchaseDate.setOnClickListener(v -> {
            btnCloseAddRecord.setVisibility(View.INVISIBLE);
            fragmentDatePicker.setVisibility(View.VISIBLE);
        });

        // Cancel date selection
        FloatingActionButton btnCancelDateSelection = view.findViewById(R.id.new_record_data_picker_cancel);
        btnCancelDateSelection.setOnClickListener(v -> {
            btnCloseAddRecord.setVisibility(View.VISIBLE);
            fragmentDatePicker.setVisibility(View.INVISIBLE);
        });

        // Confirm new date selection
        FloatingActionButton btnConfirmDateSelection = view.findViewById(R.id.new_record_data_picker_confirm);
        DatePicker datePicker = view.findViewById(R.id.new_record_data_picker);
        btnConfirmDateSelection.setOnClickListener(v -> {
            int day = datePicker.getDayOfMonth();
            int month = datePicker.getMonth();
            int year = datePicker.getYear();
            Calendar cal = Calendar.getInstance();
            cal.set(year, month, day);
//            df.setCalendar(cal);
            String formatted = df.format(cal.getTime());
            dateTime.setText(formatted);
            // close popup
            btnCloseAddRecord.setVisibility(View.VISIBLE);
            fragmentDatePicker.setVisibility(View.INVISIBLE);
        });

        // Submit button handle.
        FloatingActionButton btnSubmitRecord = view.findViewById(R.id.bnt_confirm_new_record);
        EditText paidAmount = view.findViewById(R.id.new_record_paid_amount);

        btnSubmitRecord.setOnClickListener(v -> {
            if (interactionListener != null) {
                String amount = paidAmount.getEditableText().toString();
                paidAmount.setText(null, TextView.BufferType.EDITABLE);

                String dt = dateTime.getText().toString();
                dateTime.setText(daterFormatted);

                interactionListener.onConfirmSubmission(amount, dt);
            }
        });
    }
}