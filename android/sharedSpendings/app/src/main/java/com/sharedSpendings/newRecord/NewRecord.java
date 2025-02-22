package com.sharedSpendings.newRecord;

import androidx.annotation.NonNull;

import java.util.NoSuchElementException;

public class NewRecord {
    private Long id;
    private String user_id;
    private Double amount;
    private String dt;

    public NewRecord(String user_id, Double amount, String dt) {
        this.user_id = user_id;
        this.amount = amount;
        this.dt = dt;
    }

    public NewRecord(Long id, String user_id, Double amount, String dt) {
        this.id = id;
        this.user_id = user_id;
        this.amount = amount;
        this.dt = dt;
    }

    public Long getId() {
        if (id != null) {
            return id;
        }
        throw new NoSuchElementException("The element is not inserted into the database");
    }

    public void setId(Long id) {
        this.id = id;
    }

    public String getUser_id() {
        return user_id;
    }

    public void setUser_id(String user_id) {
        this.user_id = user_id;
    }

    public Double getAmount() {
        return amount;
    }

    public void setAmount(Double amount) {
        this.amount = amount;
    }

    public String getDt() {
        return dt;
    }

    public void setDt(String dt) {
        this.dt = dt;
    }

    @NonNull
    @Override
    public String toString() {
        return "NewRecord{" +
                "id='" + id + '\'' +
                "user_id='" + user_id + '\'' +
                ", amount=" + amount +
                ", dt='" + dt + '\'' +
                '}';
    }
}
