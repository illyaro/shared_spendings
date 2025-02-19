package com.sharedSpendings.newRecord;

import java.util.Date;

public interface OnAddRecordInteractionListener {
    void onCloseAddRecord();
    void onConfirmSubmission(Double amount, Date datetime);
}
