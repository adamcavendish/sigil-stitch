async Task<string> FetchDataAsync(string url) {
    using var client = new HttpClient();
    var response = await client.GetAsync(url);
    return await response.Content.ReadAsStringAsync();
}
