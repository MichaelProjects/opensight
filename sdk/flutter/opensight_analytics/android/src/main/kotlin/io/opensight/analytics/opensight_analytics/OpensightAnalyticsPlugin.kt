package io.opensight.analytics.opensight_analytics

import android.app.Activity
import androidx.annotation.NonNull
import android.content.Context
import android.util.DisplayMetrics
import android.content.res.AssetManager;

import java.io.BufferedReader
import java.io.IOException
import java.io.InputStream
import java.io.InputStreamReader
import android.os.Build

import java.io.File

import java.util.Locale

import io.flutter.embedding.engine.plugins.FlutterPlugin
import io.flutter.embedding.engine.plugins.activity.ActivityAware
import io.flutter.embedding.engine.plugins.activity.ActivityPluginBinding
import io.flutter.plugin.common.MethodCall
import io.flutter.plugin.common.MethodChannel
import io.flutter.plugin.common.MethodChannel.MethodCallHandler
import io.flutter.plugin.common.MethodChannel.Result
import io.flutter.plugin.common.PluginRegistry.Registrar


/** OpensightAnalyticsPlugin */


class OpensightAnalyticsPlugin: FlutterPlugin, MethodCallHandler, ActivityAware  {
  /// The MethodChannel that will the communication between Flutter and native Android
  ///
  /// This local reference serves to register the plugin with the Flutter Engine and unregister it
  /// when the Flutter Engine is detached from the Activity
  private lateinit var channel : MethodChannel
  private lateinit var activity:Activity
  private lateinit var context: Context

  override fun onAttachedToEngine(@NonNull flutterPluginBinding: FlutterPlugin.FlutterPluginBinding) {
    channel = MethodChannel(flutterPluginBinding.binaryMessenger, "io.opensight_analytics")
    channel.setMethodCallHandler(this)
    this.context = flutterPluginBinding.applicationContext
  }

  companion object {
    @JvmStatic
    fun registerWith(registrar: Registrar) {
      val channel = MethodChannel(registrar.messenger(), "io.opensight_analytics")
      channel.setMethodCallHandler(OpensightAnalyticsPlugin())
    }
  }

  override fun onMethodCall(@NonNull call: MethodCall, @NonNull result: Result) {
    when (call.method){
      "getPlatformVersion" -> {result.success("Android ${android.os.Build.VERSION.RELEASE}")}
      "displaysize" -> {
        result.success(getDisplaySize(activity))
      }
      "getOpensightConfig" -> {
        var conf: String = loadServiceData("opensight_service.json")
        result.success(conf)
      }
      "getLangCode" -> {
        val langCode: String? = getSystemLang()
        result.success(langCode)
      }
      "getAppVersion" -> {
        val version: String? = getAppVersion(context)
        result.success(version)
      }
      "getDeviceType" -> {
        val deviceType: String = getPhoneDeviceName()
        result.success(deviceType)
      }
      else -> {
        result.notImplemented()
      }
    }
  }
  override fun onDetachedFromEngine(@NonNull binding: FlutterPlugin.FlutterPluginBinding) {
    channel.setMethodCallHandler(null)
  }
  override fun onDetachedFromActivity() {}
  override fun onReattachedToActivityForConfigChanges(binding: ActivityPluginBinding) {
    onAttachedToActivity(binding)
  }
  override fun onAttachedToActivity(binding: ActivityPluginBinding) {
    this.activity = binding.activity
  }
  override fun onDetachedFromActivityForConfigChanges() {}
}

// here are the util classes, where the plugin gets the information from

public fun getDisplaySize(activity: Activity): String {
  val displayMetrics = DisplayMetrics()
  activity.windowManager.defaultDisplay.getMetrics(displayMetrics)
  var width = displayMetrics.widthPixels
  var height = displayMetrics.heightPixels
  return "${width}x${height}"
}

public fun loadServiceData(fileName: String): String {
  var string: String = ""
  val stringBuilder = StringBuilder()
  //var manager: AssetManager = AssetManager()
  //print(manager.list())
  return ""
}

public fun getSystemLang():String{
  val langCode: String = Locale.getDefault().getLanguage() as String
  return langCode
}
public fun getAppVersion(context: Context):String {
  val versionName: String = context.getPackageManager()
          .getPackageInfo(context.getPackageName(), 0).versionName
  return versionName
}

fun getPhoneDeviceName():String {
  val model = Build.MODEL // returns model name
  return model;
}