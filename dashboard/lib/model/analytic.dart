class AnalyticEntry {
  String sessionId;
  String applicationId;
  DateTime creationTime;
  String os;
  String deviceSize;
  bool newUser;
  String country;
  String deviceType;
  String version;
  AnalyticEntry(
      this.sessionId,
      this.applicationId,
      this.creationTime,
      this.os,
      this.deviceSize,
      this.newUser,
      this.country,
      this.deviceType,
      this.version);

  factory AnalyticEntry.fromJson(Map responseData) {
    return AnalyticEntry(
      responseData["session_id"],
      responseData["application_id"],
      DateTime.parse(responseData["creation_time"]),
      responseData["os"],
      responseData["device_size"],
      responseData["new_user"],
      responseData["country"],
      responseData["device_type"],
      responseData["version"],
    );
  }
  List<String> to_list() {
    return [
      applicationId,
      creationTime.toString(),
      os,
      deviceSize,
      newUser.toString(),
      country,
      deviceType,
      version
    ];
  }
}
