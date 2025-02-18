package com.sharedSpendings;

public interface OnAddRecordInteractionListener {
    void onCloseAddRecord();
    void onConfirmSubmission(String amount, String datetime);
}
