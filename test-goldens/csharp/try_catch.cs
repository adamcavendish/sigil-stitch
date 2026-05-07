try {
    DoRiskyOperation();
} catch (Exception ex) {
    Logger.Error(ex.Message);
} finally {
    Cleanup();
}
