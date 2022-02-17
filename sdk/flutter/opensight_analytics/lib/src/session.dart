import 'dart:isolate';

import 'package:opensight_analytics/src/nativlayer.dart';
import 'package:opensight_core/opensight_core.dart';
import 'package:flutter_isolate/flutter_isolate.dart';

int trackIntervall = 2;

enum SessionState { none, send, resumed }

class Session {
  /// [Session] is used to count teh session length and later add more features to it.
  static String id = "";
  DateTime startTime = DateTime.now();

  void sendUpdate(OpensightCore app, int length) {
    Map<String, dynamic> payload = {
      "session_id": Session.id,
      "length": length,
    };
    app.transport
        .updateData(payload, "/analytic/v1/${app.appDetails.appId}/session");
  }
}

/// the [trackIntervall] in which distance the data will be written to the disk or storage
startTracking(SendPort sendPort) async {
  Session session = Session();
  while (true) {
    await Future.delayed(Duration(seconds: trackIntervall));
    bool result = await NativeLayer.isAppInBackground();
    print(result);
    if (result == true) {
      sendPort.send({
        "event": "SEND_UPDATE",
        "start": session.startTime,
        "end": DateTime.now()
      });
    }
  }
}

Future sendReceive(SendPort port, msg) {
  ReceivePort response = ReceivePort();
  port.send([msg, response.sendPort]);
  return response.first;
}

/// if [tracking] is called, the function will create an isolate and
/// start an endless loop to write down the past time
tracking(OpensightCore app) async {
  ReceivePort receivePort = ReceivePort();
  receivePort.listen((message) async {
    if (message["event"] == "SEND_UPDATE") {
      DateTime start = message["start"];
      DateTime end = message["end"];
      int length = end.difference(start).inSeconds;
      print(length);
      //Session().sendUpdate(app, message["data"]);
    }
  });
  await FlutterIsolate.spawn(startTracking, receivePort.sendPort);
}
