package com.sharedSpendings;

import android.net.Uri;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.ImageView;
import android.widget.TextView;

import androidx.annotation.NonNull;
import androidx.recyclerview.widget.RecyclerView;

import com.sharedSpendings.newRecord.NewRecord;

import java.util.List;

import com.bumptech.glide.Glide;

public class RecordAdapter extends RecyclerView.Adapter<RecordAdapter.RecordViewHolder> {
    private List<NewRecord> recordList;
    private String profilePictureLink = "https://lh3.googleusercontent.com/a/ACg8ocI-DpFFouaj-OcsZop_h9njgWnMJIAs96Ts78zXpoyjcptX5zmK=s96-c";

    public RecordAdapter(List<NewRecord> recordList) {
        this.recordList = recordList;
    }

    static class RecordViewHolder extends RecyclerView.ViewHolder {
        TextView amount;
        TextView username;
        TextView dateTime;
        ImageView profilePicture;

        public RecordViewHolder(@NonNull View itemView){
            super(itemView);
            amount = itemView.findViewById(R.id.record_amount);
            username = itemView.findViewById(R.id.record_username);
            dateTime = itemView.findViewById(R.id.record_time);
            profilePicture = itemView.findViewById(R.id.record_profile_picture);
        }
    }

    @NonNull
    @Override
    public RecordViewHolder onCreateViewHolder(@NonNull ViewGroup parent, int viewType){
        View view = LayoutInflater.from(parent.getContext()).inflate(R.layout.record_recycler_view, parent, false);
        return new RecordViewHolder(view);
    }

    @Override
    public void onBindViewHolder(@NonNull RecordViewHolder holder, int position) {
        NewRecord record = recordList.get(position);

        holder.amount.setText(String.valueOf(record.getAmount()));
        holder.username.setText(record.getUser_id());
        holder.dateTime.setText(record.getDt());
        Glide.with(holder.profilePicture.getContext()).load(this.profilePictureLink).into(holder.profilePicture);
//
    }

    @Override
    public int getItemCount() {
        return recordList.size();
    }

    public void addRecord (NewRecord record) {
        this.recordList.add(record);
        notifyItemInserted(recordList.size() - 1);
    }

}
