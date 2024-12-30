// use pest::{iterators::Pairs, Parser};
// use polars;
// use polars::prelude::*;
use xenosphere_parser::parsers::parse_script_token;

#[cfg(test)]
#[test]
fn test_parser_with_simple_clang() {
    let token = parse_script_token(
        r##"
/****
 * 
 * This is a simple C program
 * 
 */

int w = 10;

int add(int a, int b) {
    return a + b;
}

int main() {
    int x = 5;
    int y = 10;
    // int result = add(x, y);
    if (x > y) {
        printf("x is greater than y\n");
    } else {
        printf("y is greater than x\n");
    }
    printf("Result: %d\n", result);
    return 0;
}
 
void main_func(tensor $x, tensor $y) {
    const tensor w_1 = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];
    
    $y = $x * w;
    $y = $x ++;
    $y = $x ** w + 2;
    $y**;

    return;
}

    "##,
    );
    println!("{:#?}", token);
    assert_eq!(4, 4);
}

#[cfg(test)]
#[test]
fn test_parser_looping() {
    let token = parse_script_token(
        r##"
/****
 * 
 * This is a simple C program
 * 
 */
for (int i = 0; i < 10; i++) {
    printf("i is %d\n", i);
}
for (item in items) {
    printf("i is %d\n", i);
}
    "##,
    );
    println!("{:#?}", token);
    assert_eq!(4, 4);
}

#[cfg(test)]
#[test]
fn test_parser_with_simple_cpp() {
    let token = parse_script_token(
        r##"

void swapNums(int &x, int &y) {
  int z = x;
  x = y;
  y = z;
}

int main() {
  int firstNum = 10;
  int secondNum = 20;

//   cout << "Before swap: " << "\n";
//   cout << firstNum << secondNum << "\n";

  // Call the function, which will change the values of firstNum and secondNum
  swapNums(firstNum, secondNum);

//   cout << "After swap: " << "\n";
//   cout << firstNum << secondNum << "\n";

  return 0;
}
    "##,
    );
    println!("{:#?}", token);
    assert_eq!(4, 4);
}

#[cfg(test)]
#[test]
fn test_parser_godot_cpp() {
    let token = parse_script_token(
        r##"
        // copy from godot cpp
AnimationPlayer *animation_player = null;
AnimationPlayer *SpineAnimationTrack::find_animation_player() {
	AnimationPlayer *animation_player = nullptr;
	for (int i = 0; i < get_child_count(); i++) {
		animation_player = cast_to<AnimationPlayer>(get_child(i));
		if (animation_player) {
			break;
		}
	}
	return animation_player;
}

    "##,
    );
    println!("{:#?}", token);
    assert_eq!(4, 4);
}

#[cfg(test)]
#[test]
fn test_parser_godot_cpp_1() {
    let token = parse_script_token(
        r##"
        // copy from godot cpp
bool d = e | r;
string dsd = "dsd";
string empty = "";
void SpineAnimationTrack::_bind_methods() {
	ClassDB::bind_method();
	ClassDB::bind_method(D_METHOD("set_loop", "loop"), &SpineAnimationTrack::set_loop);
	ClassDB::bind_method(D_METHOD("get_loop"), &SpineAnimationTrack::get_loop);

	PropertyInfo("animation_name", PROPERTY_HINT_NONE, "");
	PropertyInfo(Variant::STRING, "animation_name", PROPERTY_HINT_NONE, "");
	PropertyInfo(Variant::STRING, "animation_name", PROPERTY_HINT_NONE, "", PROPERTY_USAGE_STORAGE | PROPERTY_USAGE_INTERNAL | PROPERTY_USAGE_NOEDITOR);
	ADD_PROPERTY(PropertyInfo(Variant::STRING, "animation_name", PROPERTY_HINT_NONE, "", PROPERTY_USAGE_STORAGE | PROPERTY_USAGE_INTERNAL | PROPERTY_USAGE_NOEDITOR), "set_animation_name", "get_animation_name");
	ADD_PROPERTY(PropertyInfo(Variant::BOOL, "loop", PROPERTY_HINT_NONE, "", PROPERTY_USAGE_STORAGE | PROPERTY_USAGE_INTERNAL | PROPERTY_USAGE_NOEDITOR), "set_loop", "get_loop");
}

    "##,
    );
    println!("{:#?}", token);
    assert_eq!(4, 4);
}

#[cfg(test)]
#[test]
fn test_parser_godot_cpp_2() {
    let token = parse_script_token(
        r##"

cast_to<AnimationPlayer>(get_child(i));
Object::cast_to(variant_sprite);
Object::cast_to<SpineSprite>(variant_sprite);
sprite = Object::cast_to(variant_sprite);
        // copy from godot cpp
void SpineAnimationTrack::update_animation_state(const Variant &variant_sprite) {
	if (track_index < 0) return;
	sprite = Object::cast_to<SpineSprite>(variant_sprite);
	if (!sprite) return;
	if (!sprite->get_skeleton_data_res().is_valid() || !sprite->get_skeleton_data_res()->is_skeleton_data_loaded()) return;
	if (!sprite->get_skeleton().is_valid() || !sprite->get_animation_state().is_valid()) return;
	spine::AnimationState *animation_state = sprite->get_animation_state()->get_spine_object();
	if (!animation_state) return;
	spine::Skeleton *skeleton = sprite->get_skeleton()->get_spine_object();
	if (!skeleton) return;
	AnimationPlayer *animation_player = find_animation_player();
	if (!animation_player) return;

	if (Engine::get_singleton()->is_editor_hint()) {
#ifdef TOOLS_ENABLED
		if (blend_tree_mode) {
			AnimationTreeEditor *tree_editor = AnimationTreeEditor::get_singleton();
			// When the animation tree dock is no longer visible, bail.
			if (!tree_editor->is_visible_in_tree()) {
				skeleton->setToSetupPose();
				animation_state->clearTracks();
				animation_state->setTimeScale(1);
				return;
			}
			auto current_entry = animation_state->getCurrent(track_index);
			bool should_set_mix = mix_duration >= 0;
			bool should_set_animation = !current_entry || (animation_name != current_entry->getAnimation()->getName().buffer() || current_entry->getLoop() != loop);

			if (should_set_animation) {
				if (!EMPTY(animation_name)) {
					auto entry = animation_state->setAnimation(track_index, SPINE_STRING(animation_name), loop);
					if (should_set_mix) entry->setMixDuration(mix_duration);

					entry->setHoldPrevious(hold_previous);
					entry->setReverse(reverse);
					entry->setShortestRotation(shortest_rotation);
					entry->setTimeScale(time_scale);
					entry->setAlpha(alpha);
					entry->setAttachmentThreshold(attachment_threshold);
					entry->setDrawOrderThreshold(draw_order_threshold);
					entry->setMixBlend((spine::MixBlend) mix_blend);

					if (debug) print_line(String("Setting animation {0} with mix_duration {1} on track {2} on {3}").format(varray(animation_name, mix_duration, track_index, sprite->get_name())).utf8().ptr());
				} else {
					if (!current_entry || (String("<empty>") != current_entry->getAnimation()->getName().buffer())) {
						auto entry = animation_state->setEmptyAnimation(track_index, should_set_mix ? mix_duration : 0);
						entry->setTrackEnd(FLT_MAX);
						if (debug) print_line(String("Setting empty animation with mix_duration {0} on track {1} on {2}").format(varray(mix_duration, track_index, sprite->get_name())).utf8().ptr());
					}
				}
			}
			return;
		}

		// When the animation dock is no longer visible or we aren't being
		// keyed in the current animation, bail.
#if VERSION_MAJOR > 3
		auto player_editor = AnimationPlayerEditor::get_singleton();
#else
		auto player_editor = AnimationPlayerEditor::singleton;
#endif
		if (!player_editor->is_visible_in_tree()) {
			skeleton->setToSetupPose();
			animation_state->clearTracks();
			animation_state->setTimeScale(1);
			return;
		}

		// Check if the player is actually editing an animation for which there is a track
		// for us.
		Ref<Animation> edited_animation = player_editor->get_track_editor()->get_current_animation();
		if (!edited_animation.is_valid()) {
			skeleton->setToSetupPose();
			animation_state->clearTracks();
			animation_state->setTimeScale(1);
			return;
		}

		int found_track_index = -1;
		auto scene_path = EditorNode::get_singleton()->get_edited_scene()->get_path();
		auto animation_player_path = scene_path.rel_path_to(animation_player->get_path());
		for (int i = 0; i < edited_animation->get_track_count(); i++) {
			auto path = edited_animation->track_get_path(i);
			if (path == animation_player_path) {
				found_track_index = i;
				break;
			}
		}

		// if we are track 0, set the skeleton to the setup pose
		// and the animation state time scale to 0, as we are
		// setting track times manually. Also, kill anything
		// currently in the track.
		if (track_index == 0) {
			skeleton->setToSetupPose();
			animation_state->setTimeScale(0);
		}
		animation_state->clearTrack(track_index);
		if (found_track_index == -1) return;

		// If no animation is set or it's set to "[stop]", we are done.
		if (EMPTY(animation_name) || animation_name == "[stop]") return;

		// If there's no keys on the timeline for this track, we are done.
		if (edited_animation->track_get_key_count(found_track_index) == 0) return;

		// Find the key in the track that matches the editor's playback position
		auto playback_position = player_editor->get_player()->get_current_animation_position();
		int key_index = -1;
		for (int i = 0; i < edited_animation->track_get_key_count(found_track_index); i++) {
			float key_time = edited_animation->track_get_key_time(found_track_index, i);
			if (key_time <= playback_position) {
				key_index = i;
			} else {
				// epsilon compare key and playback time, as playback time is imprecise
				if (fabs(key_time - playback_position) < edited_animation->get_step()) {
					key_index = i;
				}
				break;
			}
		}

		// No key found? bail.
		if (key_index == -1) return;

		// Get the animation from our player for the key
		float key_time = edited_animation->track_get_key_time(found_track_index, key_index);
		String key_value = edited_animation->track_get_key_value(found_track_index, key_index);
		Ref<Animation> keyed_animation = animation_player->get_animation(key_value);
		if (!keyed_animation.is_valid()) return;

		// Calculate the track time and setup the track entry based on the currently keyed
		// properties.
		float track_time = (playback_position - key_time) * time_scale;
		if (track_time < 0) track_time = 0;
		auto entry = animation_state->setAnimation(track_index, SPINE_STRING(animation_name), loop);
		entry->setMixDuration(0);
		entry->setTrackTime(track_time);

		entry->setHoldPrevious(hold_previous);
		entry->setReverse(reverse);
		entry->setShortestRotation(shortest_rotation);
		entry->setAlpha(alpha);
		entry->setAttachmentThreshold(attachment_threshold);
		entry->setDrawOrderThreshold(draw_order_threshold);
		entry->setMixBlend((spine::MixBlend) mix_blend);
#endif
	} else {
		if (animation_player->is_playing()) {
			auto current_entry = animation_state->getCurrent(track_index);
			bool should_set_mix = mix_duration >= 0;
			bool should_set_animation = !current_entry || (animation_name != current_entry->getAnimation()->getName().buffer() || current_entry->getLoop() != loop);

			if (should_set_animation) {
				if (!EMPTY(animation_name)) {
					auto entry = animation_state->setAnimation(track_index, SPINE_STRING(animation_name), loop);
					if (should_set_mix) entry->setMixDuration(mix_duration);

					entry->setHoldPrevious(hold_previous);
					entry->setReverse(reverse);
					entry->setShortestRotation(shortest_rotation);
					entry->setTimeScale(time_scale);
					entry->setAlpha(alpha);
					entry->setAttachmentThreshold(attachment_threshold);
					entry->setDrawOrderThreshold(draw_order_threshold);
					entry->setMixBlend((spine::MixBlend) mix_blend);

					if (debug) print_line(String("Setting animation {0} with mix_duration {1} on track {2} on {3}").format(varray(animation_name, mix_duration, track_index, sprite->get_name())).utf8().ptr());
				} else {
					if (!current_entry || (String("<empty>") != current_entry->getAnimation()->getName().buffer())) {
						auto entry = animation_state->setEmptyAnimation(track_index, should_set_mix ? mix_duration : 0);
						entry->setTrackEnd(FLT_MAX);
						if (debug) print_line(String("Setting empty animation with mix_duration {0} on track {1} on {2}").format(varray(mix_duration, track_index, sprite->get_name())).utf8().ptr());
					}
				}
			}
		}
	}
}


    "##,
    );
    println!("{:#?}", token);
    assert_eq!(4, 4);
}
