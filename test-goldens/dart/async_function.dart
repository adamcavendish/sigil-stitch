import 'package:myapp/models/user.dart';

// User

Future<User> fetchUser(String id) async {
  return await api.fetchUser(id);
}
