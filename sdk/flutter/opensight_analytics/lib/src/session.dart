import 'dart:isolate';

import 'package:opensight_analytics/src/nativlayer.dart';
import 'package:opensight_core/opensight_core.dart';

import 'persistence.dart';
import 'package:flutter_isolate/flutter_isolate.dart';

int trackIntervall = 2;

class Session {
  /// [Session] is used to count teh session length and later add more features to it.
  static String id = "";
  DateTime startTime = DateTime.now();
  int length = 0;

  void increaseLength() {
    length += trackIntervall;
  }

  void sendUpdate(OpensightCore app, int length) {
    Map<String, dynamic> payload = {
      "session_id": Session.id,
      "length": length,
    };
    print(payload);
    app.transport
        .updateData(payload, "/analytic/v1/${app.appDetails.appId}/session");
  }

  int store() {
    return length;
  }
}

/// the [trackIntervall] in which distance the data will be written to the disk or storage
startTracking(SendPort sendPort) async {
  Session session = Session();
  while (true) {
    await Future.delayed(Duration(seconds: trackIntervall));

    sendPort.send({"event": "UPDATE_SESSION", "data": session.length});
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
    if (message["event"] == "UPDATE_SESSION") {
      print("Disptach update to server");
      var result = await NativeLayer.isApplicationActive();
      print(result);
      //Session().sendUpdate(app, message["data"]);
    }
  });
  FlutterIsolate.spawn(startTracking, receivePort.sendPort);
}
