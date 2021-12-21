import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:opensight_core/opensight_core.dart';

void main() {
  const MethodChannel channel = MethodChannel('opensight_core');

  TestWidgetsFlutterBinding.ensureInitialized();

  setUp(() {
    channel.setMockMethodCallHandler((MethodCall methodCall) async {
      return '42';
    });
  });

  tearDown(() {
    channel.setMockMethodCallHandler(null);
  });

  test('getPlatformVersion', () async {
    expect(await OpensightCore.platformVersion, '42');
  });
}
