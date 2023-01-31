class Urls {
  static String host = '';
  static String coreUrl = "";
  static String analyticUrl = "";

  // V1 Core Endpoints
  static Uri getAllApplications = Uri.parse('$coreUrl/core/v1/application');

  static Uri getApplication(String appId) {
    return Uri.parse('$coreUrl/core/v1/application/$appId');
  }

  static Uri login = Uri.parse('$coreUrl/core/v1/login');
  static Uri createApplication = Uri.parse('$coreUrl/core/v1/application');

  // V1 Analytics Endpoints
  static Uri getAllAnalytics(String appId, int limit, int start, int end) {
    return Uri.parse(
        '$analyticUrl/analytic/v1/$appId/session?limit=$limit&start=$start&end=$end');
  }

  static Uri getAnalyseNewUser(String appid, int startFrame, int endFrame) {
    return Uri.parse(
        '$analyticUrl/analytic/v1/$appid/analyse/user/new?start=$startFrame&end=$endFrame');
  }

  static Uri getAnalyseUser(String appid, int startFrame, int endFrame) {
    return Uri.parse(
        '$analyticUrl/analytic/v1/$appid/analyse/user?start=$startFrame&end=$endFrame');
  }

  static Uri getAnalyseDisplaySize(String appid, int startFrame, int endFrame) {
    return Uri.parse(
        '$analyticUrl/analytic/v1/$appid/analyse/display/size?start=$startFrame&end=$endFrame');
  }

  static Uri getSessionCount(String appid, int startFrame, int endFrame) {
    return Uri.parse(
        '$analyticUrl/analytic/v1/$appid/session/count?start=$startFrame&end=$endFrame');
  }

  static Uri getSessionLengthHistory(
      String appid, int startFrame, int endFrame) {
    return Uri.parse(
        '$analyticUrl/analytic/v1/$appid/analyse/user/session?start=$startFrame&end=$endFrame');
  }

  static Uri getAppVersion(String appid, int startFrame, int endFrame) {
    return Uri.parse(
        '$analyticUrl/analytic/v1/$appid/analyse/app/version?start=$startFrame&end=$endFrame');
  }
}
