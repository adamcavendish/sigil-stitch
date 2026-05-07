try {
    DoSomething();
}
catch (Exception ex) {
    Log(ex.Message);
}
finally {
    Cleanup();
}
