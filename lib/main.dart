import 'package:flutter/material.dart';
import 'package:qspace_pages/qspace_pages.dart';
import 'client/qspace_press/client_config.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  runApp(AppRoot(config: kQSpacePressClientConfig));
}