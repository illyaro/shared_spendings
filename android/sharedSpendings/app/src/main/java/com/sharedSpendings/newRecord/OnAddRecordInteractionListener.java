package com.sharedSpendings.newRecord;

public interface OnAddRecordInteractionListener {
    void onCloseAddRecord();
    void onConfirmSubmission(String amount, String datetime);
}
