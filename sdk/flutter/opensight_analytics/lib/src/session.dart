import 'package:opensight_analytics/src/nativlayer.dart';
import 'package:opensight_analytics/src/persistence.dart';
import 'package:opensight_core/opensight_core.dart';

int trackIntervall = 2;

enum SessionState { started, background, resumed }

class Session {
  /// [Session] is used to count teh session length and later add more features to it.
  String id = "";
  SessionState state = SessionState.started;
  // if the app gets resumed the session length from before is stored here to update the session on the server(analytic api) later if the app gets minimized.
  int resumedLength = 0;
  DateTime startTime = DateTime.now().toUtc();
  bool isFirstToday = false;

  void sendUpdate(OpensightCore app, int length, String sessionId) {
    Map<String, dynamic> payload = {
      "session_id": sessionId,
      "length": length,
    };
    app.transport
        .updateData(payload, "/analytic/v1/${app.appDetails.appId}/session");
  }
}

/// if [tracking] is called, the function will start to track the session length
/// if the app gets minimized the session length will be stored in [Session.resumedLength]
/// and will be sent to the server unsing [Session.sendUpdate] call.
void tracking(OpensightCore app, String sessionId) async {
  Session session = Session();
  DateTime? lastLogin = await PresistencesLayer().getLastLoginDate();
  // check last login time, if the user has logged in the first time today.
  if (lastLogin != null) {
    session.isFirstToday = checkDate(lastLogin);
  }
  while (true) {
    await Future.delayed(Duration(seconds: trackIntervall));
    // check if app is in background
    bool result = await NativeLayer.isAppInBackground();
    if (result && session.state == SessionState.started) {
      session.state = SessionState.background;
      int sessionLength =
          DateTime.now().toUtc().difference(session.startTime).inSeconds;
      session.sendUpdate(app, sessionLength, sessionId);
    }
    if (result && session.state == SessionState.resumed) {
      session.state = SessionState.background;
      int sessionLength =
          DateTime.now().toUtc().difference(session.startTime).inSeconds +
              session.resumedLength;
      session.sendUpdate(app, sessionLength, sessionId);
    }
    if (!result && session.state == SessionState.background) {
      DateTime now = DateTime.now().toUtc();

      // todo this check can be enabled if the create session endpoint is ready
      //if (now.difference(session.startTime).inSeconds <= 300) {

      int sessionLength =
          DateTime.now().toUtc().difference(session.startTime).inSeconds;
      session.resumedLength = sessionLength;
      session.startTime = DateTime.now().toUtc();
      session.state = SessionState.resumed;

      //} else {
      // todo for this function is a seperate session create endpoint needed, this should be implemented in the core module in the future.
      // print("session ended");
      // int sessionLength =
      //     DateTime.now().toUtc().difference(session.startTime).inSeconds;
      // session.sendUpdate(app, sessionLength, sessionId);
      // session.state = SessionState.started;
      //}
    }
  }
}

bool checkDate(DateTime toCheck) {
  final now = DateTime.now().toUtc();
  final today = DateTime(now.year, now.month, now.day);
  final aDate = DateTime(toCheck.year, toCheck.month, toCheck.day);
  if (aDate == today) {
    return true;
  }
  return false;
}
