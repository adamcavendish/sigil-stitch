async function fetchData(url: string): Promise<Response> {
  const response = await fetch(url);
  return response.json();
}
