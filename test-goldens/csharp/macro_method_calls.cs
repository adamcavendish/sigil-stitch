var result = await client.GetAsync(url);
var content = await result.Content.ReadAsStringAsync();
Console.WriteLine(content);
