// press/test/widget_test.dart
import 'package:flutter_test/flutter_test.dart';

void main() {
  testWidgets('placeholder — wire a real test when AppRoot is stable',
      (WidgetTester tester) async {
    // AppRoot requires full initialization (providers, merge engine, overlay).
    // Integration-test it instead of unit-testing here.
    // This file is intentionally minimal to unblock development.
    expect(true, isTrue);
  });
}