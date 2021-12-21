#import "OpensightAnalyticsPlugin.h"
#if __has_include(<opensight_analytics/opensight_analytics-Swift.h>)
#import <opensight_analytics/opensight_analytics-Swift.h>
#else
// Support project import fallback if the generated compatibility header
// is not copied when this plugin is created as a library.
// https://forums.swift.org/t/swift-static-libraries-dont-copy-generated-objective-c-header/19816
#import "opensight_analytics-Swift.h"
#endif

@implementation OpensightAnalyticsPlugin
+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar>*)registrar {
  [SwiftOpensightAnalyticsPlugin registerWithRegistrar:registrar];
}
@end
