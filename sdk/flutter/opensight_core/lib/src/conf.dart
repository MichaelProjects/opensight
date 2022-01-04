class Config {
  final String? analyticsApi;
  final String? appId;
  final String? token;
  final String? name;
  final String? packageName;

  Config(
      this.analyticsApi, this.appId, this.token, this.name, this.packageName);

  factory Config.fromJson(Map data) {
    return Config(
      data["url"],
      data["app_id"],
      data["token"],
      data["name"],
      data["package_name"],
    );
  }
  factory Config.def() {
    return Config("localhost:28018", "123", "default", "123", "io.default");
  }
}
