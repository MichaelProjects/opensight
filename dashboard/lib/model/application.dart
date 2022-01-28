class Application {
  final String appID;
  final String name;
  final String packageId;
  final bool isIos;
  final bool isAndroid;

  Application({
    required this.appID,
    required this.name,
    required this.packageId,
    required this.isIos,
    required this.isAndroid,
  });
  factory Application.fromJson(Map<String, dynamic> response) {
    return Application(
      appID: response['application_id'],
      name: response['application_name'],
      packageId: response['package_name'],
      isIos: true,
      isAndroid: false,
    );
  }
  factory Application.mock() {
    return Application(
      appID: '1',
      name: 'Mock APP',
      packageId: 'com.mock',
      isIos: true,
      isAndroid: false,
    );
  }
}
