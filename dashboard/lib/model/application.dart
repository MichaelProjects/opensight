class Application {
  final String name;
  final String packageId;
  final bool isIos;
  final bool isAndroid;

  Application({
    required this.name,
    required this.packageId,
    required this.isIos,
    required this.isAndroid,
  });
  factory Application.fromJson(Map<String, dynamic> response) {
    return Application(
      name: response['name'],
      packageId: response['packageId'],
      isIos: response['isIos'],
      isAndroid: response['isAndroid'],
    );
  }
}
