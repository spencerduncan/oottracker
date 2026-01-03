using System;
using System.Collections.Generic;
using System.Drawing;
using System.Linq;
using System.Net;
using System.Net.Sockets;
using System.Runtime.InteropServices;
using System.Text;
using System.Windows.Forms;

using BizHawk.Client.Common;
using BizHawk.Client.EmuHawk;

namespace Net.Fenhl.OotAutoTracker {
    internal class Native {
        [DllImport("oottracker")] internal static extern StringHandle expected_bizhawk_version_string();
        [DllImport("oottracker")] internal static extern StringHandle running_bizhawk_version_string();
        [DllImport("oottracker")] internal static extern StringHandle version_string();
        [DllImport("oottracker")] internal static extern BoolResult update_available();
        [DllImport("oottracker")] internal static extern void bool_result_free(IntPtr bool_res);
        [DllImport("oottracker")] internal static extern bool bool_result_is_ok(BoolResult bool_res);
        [DllImport("oottracker")] internal static extern bool bool_result_unwrap(IntPtr bool_res);
        [DllImport("oottracker")] internal static extern StringHandle bool_result_debug_err(IntPtr bool_res);
        [DllImport("oottracker")] internal static extern UnitResult run_updater();
        [DllImport("oottracker")] internal static extern Config config_default();
        [DllImport("oottracker")] internal static extern OptConfigResult config_load();
        [DllImport("oottracker")] internal static extern void opt_config_result_free(IntPtr opt_cfg_res);
        [DllImport("oottracker")] internal static extern bool opt_config_result_is_ok(OptConfigResult opt_cfg_res);
        [DllImport("oottracker")] internal static extern bool opt_config_result_is_ok_some(OptConfigResult opf_cfg_res);
        [DllImport("oottracker")] internal static extern Config opt_config_result_unwrap_unwrap_or_default(IntPtr opt_cfg_res);
        [DllImport("oottracker")] internal static extern StringHandle opt_config_result_debug_err(IntPtr opt_cfg_res);
        [DllImport("oottracker")] internal static extern void config_free(IntPtr cfg);
        [DllImport("oottracker")] internal static extern bool config_update_check_is_some(Config cfg);
        [DllImport("oottracker")] internal static extern bool config_update_check(Config cfg);
        [DllImport("oottracker")] internal static extern UnitResult config_set_update_check(Config cfg, bool auto_update_check);
        [DllImport("oottracker")] internal static extern TrackerLayout config_layout(Config cfg);
        [DllImport("oottracker")] internal static extern void layout_free(IntPtr layout);
        [DllImport("oottracker")] internal static extern TrackerCell layout_cell(TrackerLayout layout, byte idx);
        [DllImport("oottracker")] internal static extern void cell_free(IntPtr cell);
        [DllImport("oottracker")] internal static extern StringHandle cell_image(ModelState model, TrackerCell cell);
        [DllImport("oottracker")] internal static extern TcpStreamResult connect_ipv4(byte[] addr);
        [DllImport("oottracker")] internal static extern TcpStreamResult connect_ipv6(byte[] addr);
        [DllImport("oottracker")] internal static extern void tcp_stream_result_free(IntPtr tcp_stream_res);
        [DllImport("oottracker")] internal static extern bool tcp_stream_result_is_ok(TcpStreamResult tcp_stream_res);
        [DllImport("oottracker")] internal static extern TcpStream tcp_stream_result_unwrap(IntPtr tcp_stream_res);
        [DllImport("oottracker")] internal static extern void tcp_stream_free(IntPtr tcp_stream);
        [DllImport("oottracker")] internal static extern StringHandle tcp_stream_result_debug_err(IntPtr tcp_stream_res);
        [DllImport("oottracker")] internal static extern void string_free(IntPtr s);
        [DllImport("oottracker")] internal static extern UnitResult tcp_stream_disconnect(IntPtr tcp_stream);
        [DllImport("oottracker")] internal static extern void unit_result_free(IntPtr unit_res);
        [DllImport("oottracker")] internal static extern bool unit_result_is_ok(UnitResult unit_res);
        [DllImport("oottracker")] internal static extern StringHandle unit_result_debug_err(IntPtr unit_res);
        [DllImport("oottracker")] internal static extern SaveResult save_from_save_data(byte[] start);
        [DllImport("oottracker")] internal static extern void save_result_free(IntPtr save_res);
        [DllImport("oottracker")] internal static extern bool save_result_is_ok(SaveResult save_res);
        [DllImport("oottracker")] internal static extern Save save_result_unwrap(IntPtr save_res);
        [DllImport("oottracker")] internal static extern Save save_default();
        [DllImport("oottracker")] internal static extern void save_free(IntPtr save);
        [DllImport("oottracker")] internal static extern StringHandle save_debug(Save save);
        [DllImport("oottracker")] internal static extern StringHandle save_result_debug_err(IntPtr save_res);
        [DllImport("oottracker")] internal static extern UnitResult save_send(TcpStream tcp_stream, Save save);
        [DllImport("oottracker")] internal static extern bool saves_equal(Save save1, Save save2);
        [DllImport("oottracker")] internal static extern SavesDiff saves_diff(Save old_save, Save new_save);
        [DllImport("oottracker")] internal static extern void saves_diff_free(IntPtr diff);
        [DllImport("oottracker")] internal static extern UnitResult saves_diff_send(TcpStream tcp_stream, IntPtr diff);
        [DllImport("oottracker")] internal static extern Knowledge knowledge_none();
        [DllImport("oottracker")] internal static extern Knowledge knowledge_vanilla();
        [DllImport("oottracker")] internal static extern void knowledge_free(IntPtr knowledge);
        [DllImport("oottracker")] internal static extern UnitResult knowledge_send(TcpStream tcp_stream, Knowledge knowledge);
        [DllImport("oottracker")] internal static extern ModelState model_new(IntPtr save, IntPtr knowledge);
        [DllImport("oottracker")] internal static extern void model_free(IntPtr model);
        [DllImport("oottracker")] internal static extern byte ram_num_ranges();
        [DllImport("oottracker")] internal static extern IntPtr ram_ranges();
        [DllImport("oottracker")] internal static extern RamResult ram_from_ranges(IntPtr[] ranges);
        [DllImport("oottracker")] internal static extern void ram_result_free(IntPtr ram_res);
        [DllImport("oottracker")] internal static extern bool ram_result_is_ok(RamResult ram_res);
        [DllImport("oottracker")] internal static extern Ram ram_result_unwrap(IntPtr ram_res);
        [DllImport("oottracker")] internal static extern StringHandle ram_result_debug_err(IntPtr ram_res);
        [DllImport("oottracker")] internal static extern void ram_free(IntPtr ram);
        [DllImport("oottracker")] internal static extern bool ram_equal(Ram ram1, Ram ram2);
        [DllImport("oottracker")] internal static extern void model_set_ram(ModelState model, Ram ram);
        [DllImport("oottracker")] internal static extern Save ram_clone_save(Ram ram);
        [DllImport("oottracker")] internal static extern void model_set_tracker_ctx(ModelState model, int length, IntPtr data);
    }

    // MM memory address constants (RDRAM addresses)
    internal static class MmAddresses {
        // MM SaveContext: 0x801ef670 (System Bus) = 0x1ef670 (RDRAM)
        internal const int MM_SAVE_ADDR = 0x1ef670;
        internal const int MM_SAVE_SIZE = 0x48d0; // 18640 bytes

        // Combo randomizer context addresses (RDRAM)
        internal const int OOT_COMBO_CONTEXT_ADDR = 0x6584;
        internal const int MM_COMBO_CONTEXT_ADDR = 0x98280;
    }

    // OoTMM ROM header detection constants
    internal static class OoTMMSignatures {
        // OoTMM ROM signature - appears in ROM name area (offset 0x20)
        // OoTMM modifies the ROM name to include "OOTMM" or similar identifiers
        internal static readonly byte[] OOTMM_NAME_SIGNATURE = Encoding.UTF8.GetBytes("OOTMM");

        // OoTMM combo ROMs may also have specific game code patterns
        // Standard N64 ROM header: 0x3B-0x3E contains game ID
        // OoTMM typically uses a modified game ID
        internal const int GAME_ID_OFFSET = 0x3B;
        internal const int ROM_NAME_OFFSET = 0x20;
        internal const int ROM_NAME_LENGTH = 0x14;

        // OoTMM-specific memory signatures for active game detection
        // These help identify which game (OoT or MM) is currently active
        internal const int ACTIVE_GAME_FLAG_ADDR = 0x6580; // RDRAM offset
        internal const int SCENE_ID_OOT_ADDR = 0x1c8545; // OoT current scene
        internal const int SCENE_ID_MM_ADDR = 0x1ef674; // MM current scene (in save context)

        // OoTMM places MM ROM data at 32 MiB offset
        internal const int MM_ROM_OFFSET = 0x2000000;

        // Scene ID ranges for distinguishing games
        internal const byte OOT_MAX_SCENE_ID = 0x65;
        internal const byte MM_MIN_VALID_SCENE = 0x00;
        internal const byte MM_MAX_SCENE_ID = 0x70;
    }

    // Game type detection
    internal enum GameType {
        Unknown,
        OcarinaOfTime,
        MajorasMask,
        Combo
    }

    // Active game within combo ROM (OoT mode vs MM mode)
    internal enum ComboActiveGame {
        OcarinaOfTime,
        MajorasMask
    }

    internal class StringHandle : SafeHandle {
        internal StringHandle() : base(IntPtr.Zero, true) {}

        public override bool IsInvalid {
            get { return this.handle == IntPtr.Zero; }
        }

        public string AsString() {
            int len = 0;
            while (Marshal.ReadByte(this.handle, len) != 0) { ++len; }
            byte[] buffer = new byte[len];
            Marshal.Copy(this.handle, buffer, 0, buffer.Length);
            return Encoding.UTF8.GetString(buffer);
        }

        protected override bool ReleaseHandle() {
            if (!this.IsInvalid) {
                Native.string_free(this.handle);
            }
            return true;
        }
    }

    internal class OptConfigResult : SafeHandle {
        internal OptConfigResult() : base(IntPtr.Zero, true) {}

        public override bool IsInvalid {
            get { return this.handle == IntPtr.Zero; }
        }

        protected override bool ReleaseHandle() {
            if (!this.IsInvalid) {
                Native.opt_config_result_free(this.handle);
            }
            return true;
        }

        internal bool IsOk() => Native.opt_config_result_is_ok(this);
        internal bool IsOkSome() => Native.opt_config_result_is_ok_some(this);

        internal Config UnwrapUnwrapOrDefault() {
            var cfg = Native.opt_config_result_unwrap_unwrap_or_default(this.handle);
            this.handle = IntPtr.Zero; // opt_config_result_unwrap_unwrap_or_default takes ownership
            return cfg;
        }

        internal StringHandle DebugErr() {
            var err = Native.opt_config_result_debug_err(this.handle);
            this.handle = IntPtr.Zero; // opt_config_result_debug_err takes ownership
            return err;
        }
    }

    internal class Config : SafeHandle {
        internal Config() : base(IntPtr.Zero, true) {}

        public override bool IsInvalid {
            get { return this.handle == IntPtr.Zero; }
        }

        protected override bool ReleaseHandle() {
            if (!this.IsInvalid) {
                Native.config_free(this.handle);
            }
            return true;
        }

        internal TrackerLayout Layout() => Native.config_layout(this);
        internal bool UpdateCheckIsSome() => Native.config_update_check_is_some(this);
        internal bool UpdateCheck() => Native.config_update_check(this);
        internal UnitResult SetUpdateCheck(bool auto_update_check) => Native.config_set_update_check(this, auto_update_check);
    }

    internal class BoolResult : SafeHandle {
        internal BoolResult() : base(IntPtr.Zero, true) {}

        public override bool IsInvalid {
            get { return this.handle == IntPtr.Zero; }
        }

        protected override bool ReleaseHandle() {
            if (!this.IsInvalid) {
                Native.bool_result_free(this.handle);
            }
            return true;
        }

        internal bool IsOk() => Native.bool_result_is_ok(this);

        internal bool Unwrap() {
            var inner = Native.bool_result_unwrap(this.handle);
            this.handle = IntPtr.Zero; // bool_result_unwrap takes ownership
            return inner;
        }

        internal StringHandle DebugErr() {
            var err = Native.bool_result_debug_err(this.handle);
            this.handle = IntPtr.Zero; // bool_result_debug_err takes ownership
            return err;
        }
    }

    internal class TrackerLayout : SafeHandle {
        internal TrackerLayout() : base(IntPtr.Zero, true) {}

        public override bool IsInvalid {
            get { return this.handle == IntPtr.Zero; }
        }

        protected override bool ReleaseHandle() {
            if (!this.IsInvalid) {
                Native.layout_free(this.handle);
            }
            return true;
        }

        internal TrackerCell Cell(byte idx) => Native.layout_cell(this, idx);
    }

    internal class TrackerCell : SafeHandle {
        internal TrackerCell() : base(IntPtr.Zero, true) {}

        public override bool IsInvalid {
            get { return this.handle == IntPtr.Zero; }
        }

        protected override bool ReleaseHandle() {
            if (!this.IsInvalid) {
                Native.cell_free(this.handle);
            }
            return true;
        }

        public StringHandle Image(ModelState model) => Native.cell_image(model, this);
    }

    internal class TcpStreamResult : SafeHandle {
        internal TcpStreamResult() : base(IntPtr.Zero, true) {}

        internal static TcpStreamResult Connect(IPAddress addr) {
            return addr.AddressFamily switch {
                AddressFamily.InterNetwork => Native.connect_ipv4(addr.GetAddressBytes().ToArray()),
                AddressFamily.InterNetworkV6 => Native.connect_ipv6(addr.GetAddressBytes().ToArray()),
                _ => throw new NotImplementedException("can only connect to an IPv4 or IPv6 address"),
            };
        }

        public override bool IsInvalid {
            get { return this.handle == IntPtr.Zero; }
        }

        protected override bool ReleaseHandle() {
            if (!this.IsInvalid) {
                Native.tcp_stream_result_free(this.handle);
            }
            return true;
        }

        internal bool IsOk() => Native.tcp_stream_result_is_ok(this);

        internal TcpStream Unwrap() {
            var tcp_stream = Native.tcp_stream_result_unwrap(this.handle);
            this.handle = IntPtr.Zero; // tcp_stream_result_unwrap takes ownership
            return tcp_stream;
        }

        internal StringHandle DebugErr() {
            var err = Native.tcp_stream_result_debug_err(this.handle);
            this.handle = IntPtr.Zero; // tcp_stream_result_debug_err takes ownership
            return err;
        }
    }

    internal class TcpStream : SafeHandle {
        internal TcpStream() : base(IntPtr.Zero, true) {}

        public override bool IsInvalid {
            get { return this.handle == IntPtr.Zero; }
        }

        protected override bool ReleaseHandle() {
            if (!this.IsInvalid) {
                Native.tcp_stream_free(this.handle);
            }
            return true;
        }

        internal UnitResult Disconnect() {
            var unit_res = Native.tcp_stream_disconnect(this.handle);
            this.handle = IntPtr.Zero; // tcp_stream_disconnect takes ownership
            return unit_res;
        }
    }

    internal class UnitResult : SafeHandle {
        internal UnitResult() : base(IntPtr.Zero, true) {}

        public override bool IsInvalid {
            get { return this.handle == IntPtr.Zero; }
        }

        protected override bool ReleaseHandle() {
            if (!this.IsInvalid) {
                Native.unit_result_free(this.handle);
            }
            return true;
        }

        internal bool IsOk() => Native.unit_result_is_ok(this);

        internal StringHandle DebugErr() {
            var err = Native.unit_result_debug_err(this.handle);
            this.handle = IntPtr.Zero; // unit_result_debug_err takes ownership
            return err;
        }
    }

    internal class SaveResult : SafeHandle {
        internal SaveResult() : base(IntPtr.Zero, true) {}

        public override bool IsInvalid {
            get { return this.handle == IntPtr.Zero; }
        }

        protected override bool ReleaseHandle() {
            if (!this.IsInvalid) {
                Native.save_result_free(this.handle);
            }
            return true;
        }

        internal Save Unwrap() {
            var save = Native.save_result_unwrap(this.handle);
            this.handle = IntPtr.Zero; // save_result_unwrap takes ownership
            return save;
        }

        internal bool IsOk() => Native.save_result_is_ok(this);

        internal StringHandle DebugErr() {
            var err = Native.save_result_debug_err(this.handle);
            this.handle = IntPtr.Zero; // save_result_debug_err takes ownership
            return err;
        }
    }

    internal class Save : SafeHandle {
        internal Save() : base(IntPtr.Zero, true) {}

        public override bool IsInvalid {
            get { return this.handle == IntPtr.Zero; }
        }

        protected override bool ReleaseHandle() {
            if (!this.IsInvalid) {
                Native.save_free(this.handle);
            }
            return true;
        }

        public IntPtr Move() {
            var ptr = this.handle;
            this.handle = IntPtr.Zero;
            return ptr;
        }

        internal bool Equals(Save other) => Native.saves_equal(this, other);
        internal SavesDiff Diff(Save other) => Native.saves_diff(this, other);
        internal UnitResult Send(TcpStream tcp_stream) => Native.save_send(tcp_stream, this);
        internal StringHandle Debug() => Native.save_debug(this);
    }

    internal class SavesDiff : SafeHandle {
        internal SavesDiff() : base(IntPtr.Zero, true) {}

        public override bool IsInvalid {
            get { return this.handle == IntPtr.Zero; }
        }

        protected override bool ReleaseHandle() {
            if (!this.IsInvalid) {
                Native.saves_diff_free(this.handle);
            }
            return true;
        }

        internal UnitResult Send(TcpStream tcp_stream) {
            var unit_res = Native.saves_diff_send(tcp_stream, this.handle);
            this.handle = IntPtr.Zero; // saves_diff_send takes ownership
            return unit_res;
        }
    }

    internal class Knowledge : SafeHandle {
        internal Knowledge() : base(IntPtr.Zero, true) {}

        public override bool IsInvalid {
            get { return this.handle == IntPtr.Zero; }
        }

        protected override bool ReleaseHandle() {
            if (!this.IsInvalid) {
                Native.knowledge_free(this.handle);
            }
            return true;
        }

        public IntPtr Move() {
            var ptr = this.handle;
            this.handle = IntPtr.Zero;
            return ptr;
        }

        internal UnitResult Send(TcpStream tcp_stream) => Native.knowledge_send(tcp_stream, this);
    }

    internal class ModelState : SafeHandle {
        internal ModelState() : base(IntPtr.Zero, true) {}

        internal static ModelState FromSaveAndKnowledge(Save save, Knowledge knowledge) {
            var save_ptr = save.Move();
            var knowledge_ptr = knowledge.Move();
            return Native.model_new(save_ptr, knowledge_ptr);
        }

        public override bool IsInvalid {
            get { return this.handle == IntPtr.Zero; }
        }

        protected override bool ReleaseHandle() {
            if (!this.IsInvalid) {
                Native.model_free(this.handle);
            }
            return true;
        }

        public void SetRam(Ram ram) => Native.model_set_ram(this, ram);

        internal void SetAutoTrackerContext(IMemoryApi memoryApi, long addr, int length) {
            IntPtr data = Marshal.AllocHGlobal(length);
            Marshal.Copy(memoryApi.ReadByteRange(addr, length, "System Bus").ToArray(), 0, data, length);
            Native.model_set_tracker_ctx(this, length, data);
        }
    }

    internal class RamResult : SafeHandle {
        internal RamResult() : base(IntPtr.Zero, true) {}

        public override bool IsInvalid {
            get { return this.handle == IntPtr.Zero; }
        }

        protected override bool ReleaseHandle() {
            if (!this.IsInvalid) {
                Native.ram_result_free(this.handle);
            }
            return true;
        }

        internal bool IsOk() => Native.ram_result_is_ok(this);

        internal Ram Unwrap() {
            var ram = Native.ram_result_unwrap(this.handle);
            this.handle = IntPtr.Zero; // ram_result_unwrap takes ownership
            return ram;
        }

        internal StringHandle DebugErr() {
            var err = Native.ram_result_debug_err(this.handle);
            this.handle = IntPtr.Zero; // ram_result_debug_err takes ownership
            return err;
        }
    }

    internal class Ram : SafeHandle {
        internal Ram() : base(IntPtr.Zero, true) {}

        public override bool IsInvalid {
            get { return this.handle == IntPtr.Zero; }
        }

        protected override bool ReleaseHandle() {
            if (!this.IsInvalid) {
                Native.ram_free(this.handle);
            }
            return true;
        }

        public IntPtr Move() {
            var ptr = this.handle;
            this.handle = IntPtr.Zero;
            return ptr;
        }

        internal Save CloneSave() => Native.ram_clone_save(this);
        internal bool Equals(Ram other) => Native.ram_equal(this, other);
    }

    class RawRam {
        internal byte num_ranges;
        internal int[] ranges;
        private string[] range_hashes;
        internal byte[][] range_data;

        internal RawRam(IMemoryApi memoryApi) {
            this.num_ranges = Native.ram_num_ranges();
            this.ranges = new int[2 * num_ranges];
            Marshal.Copy(Native.ram_ranges(), this.ranges, 0, 2 * this.num_ranges);
            this.range_hashes = new string[this.num_ranges];
            this.range_data = new byte[this.num_ranges][];
            for (byte i = 0; i < this.num_ranges; i++) {
                this.range_hashes[i] = memoryApi.HashRegion(this.ranges[2 * i], this.ranges[2 * i + 1], "RDRAM");
                this.range_data[i] = memoryApi.ReadByteRange(this.ranges[2 * i], this.ranges[2 * i + 1], "RDRAM").ToArray();
            }
        }

        internal bool Update(IMemoryApi memoryApi) {
            bool changed = false;
            for (byte i = 0; i < this.num_ranges; i++) {
                var new_hash = memoryApi.HashRegion(this.ranges[2 * i], this.ranges[2 * i + 1], "RDRAM");
                if (new_hash != this.range_hashes[i]) {
                    changed = true;
                    this.range_hashes[i] = new_hash;
                    this.range_data[i] = memoryApi.ReadByteRange(this.ranges[2 * i], this.ranges[2 * i + 1], "RDRAM").ToArray();
                }
            }
            return changed;
        }

        internal RamResult ToRam() {
            IntPtr[] range_data = new IntPtr[this.num_ranges];
            for (byte i = 0; i < this.num_ranges; i++) {
                range_data[i] = Marshal.AllocHGlobal(this.ranges[2 * i + 1]);
                Marshal.Copy(this.range_data[i], 0, range_data[i], this.ranges[2 * i + 1]);
            }
            var ram_res = Native.ram_from_ranges(range_data);
            for (byte i = 0; i < this.num_ranges; i++) {
                Marshal.FreeHGlobal(range_data[i]);
            }
            return ram_res;
        }
    }

    [ExternalTool("OoT auto-tracker", Description = "An auto-tracking plugin for Fenhl's OoT tracker")]
    [ExternalToolEmbeddedIcon("Net.Fenhl.OotAutoTracker.Resources.icon.ico")]
    public sealed class MainForm : ToolFormBase, IExternalToolForm {
        private PictureBox[] cells = new PictureBox[52];
        private Label label_Version = new Label();
        private Button button_Update = new Button();
        private Label label_Update = new Label();
        private Label label_Game = new Label();
        //private Label label_Connection = new Label();
        private Label label_Save = new Label();
        private Label label_Help = new Label();
        private Button button_Close_Menu = new Button();

        public ApiContainer? _apiContainer { get; set; }
        private ApiContainer APIs => _apiContainer ?? throw new NullReferenceException();

        public override bool BlocksInputWhenFocused { get; } = false;
        protected override string WindowTitleStatic => "OoT auto-tracker";

        public override bool AskSaveChanges() => true;

        private bool initialized = false;
        private Config cfg = Native.config_default();
        private bool isVanilla;
        private GameType detectedGame = GameType.Unknown;
        //private TcpStream? stream;
        private uint? autoTrackerContextAddr;
        private uint autoTrackerContextVersion = 0;
        private RawRam? rawRam;
        private Ram? prevRam;
        private List<byte> prevSaveData = new List<byte>();
        private byte[]? prevMmSaveData;
        private Save? prevSave;
        private ModelState model = ModelState.FromSaveAndKnowledge(Native.save_default(), Native.knowledge_none());
        private string[] cellImages = new string[52];

        private bool gameOk = false;
        //private bool connectionOk = false;
        private bool saveOk = false;

        // OoTMM combo mode state tracking
        private bool isComboRom = false;
        private ComboActiveGame comboActiveGame = ComboActiveGame.OcarinaOfTime;
        private bool comboDetectedFromRomHeader = false;
        private int comboContextCheckFrames = 0;
        private const int COMBO_CONTEXT_CHECK_INTERVAL = 30; // Check every 30 frames

        public MainForm() {
            SuspendLayout();
            this.FormBorderStyle = FormBorderStyle.FixedSingle;
            this.MaximizeBox = false;
            this.ClientSize = new Size(720, 896);
            this.Icon = new Icon(typeof(MainForm).Assembly.GetManifestResourceStream("Net.Fenhl.OotAutoTracker.Resources.icon.ico"));
            this.BackColor = Color.Black;
            this.AutoScaleMode = AutoScaleMode.Dpi;

            // cells
            for (int i = 0; i < 52; i++) {
                PictureBox cell = new PictureBox();
                this.cells[i] = cell;
                cell.Location = i switch {
                    _ when i < 6 => new Point(120 * i + 10, 10),
                    _ when i < 14 => new Point(120 * (i % 6) + 10, 120 * (i / 6) - 54),
                    _ when i < 17 => new Point(80 * (i - 14) + 250, 186),
                    _ when i < 19 => new Point(120 * ((i - 1) % 6) + 10, 120 * ((i - 1) / 6) - 54),
                    _ when i < 22 => new Point(80 * (i - 19) + 250, 226),
                    _ => new Point(120 * ((i - 4) % 6) + 10, 120 * ((i - 4) / 6) - 54),
                };
                cell.Size = i switch {
                    _ when i < 6 => new Size(100, 36),
                    14 or 15 or 16 => new Size(60, 20),
                    19 or 20 or 21 => new Size(60, 60),
                    _ => new Size(100, 100),
                };
                cell.SizeMode = PictureBoxSizeMode.StretchImage;
                //TODO accessibility metadata?
                if (i >= 6 && i < 12) {
                    cell.Click += new EventHandler((object sender, EventArgs e) => {
                        MouseEventArgs me = (MouseEventArgs) e;
                        if (me.Button == MouseButtons.Right) {
                            this.label_Version.Visible = true;
                            this.button_Update.Visible = true;
                            this.label_Update.Visible = true;
                            this.label_Game.Visible = true;
                            //this.label_Connection.Visible = true;
                            this.label_Save.Visible = true;
                            this.label_Help.Visible = true;
                            this.button_Close_Menu.Visible = true;
                            foreach (PictureBox cell in this.cells) {
                                cell.Visible = false;
                            }
                            this.FormBorderStyle = FormBorderStyle.Sizable;
                            this.MaximizeBox = true;
                        }
                    });
                }
                this.Controls.Add(cell);
            }
            UpdateCells();

            // label_Version
            this.label_Version.ForeColor = Color.White;
            this.label_Version.AutoSize = true;
            this.label_Version.Location = new Point(12, 9);
            this.label_Version.Name = "label_Version";
            this.label_Version.Size = new Size(96, 25);
            this.label_Version.TabIndex = 0;
            this.label_Version.Text = $"OoT auto-tracker version {Native.version_string().AsString()} for BizHawk version {Native.expected_bizhawk_version_string().AsString()}";
            this.label_Version.Visible = false;
            this.Controls.Add(this.label_Version);

            // button_Update
            this.button_Update.ForeColor = Color.White;
            this.button_Update.AutoSize = true;
            this.button_Update.Location = new Point(12, 34);
            this.button_Update.Name = "button_Update";
            this.button_Update.Size = new Size(96, 25);
            this.button_Update.TabIndex = 1;
            this.button_Update.Text = "Check for updates…";
            this.button_Update.Visible = false;
            this.button_Update.Click += new EventHandler((object sender, EventArgs e) => {
                CheckForUpdates();
            });
            this.Controls.Add(this.button_Update);

            // label_Update
            this.label_Update.ForeColor = Color.White;
            this.label_Update.AutoSize = true;
            this.label_Update.Location = new Point(222, 39);
            this.label_Update.Name = "label_Update";
            this.label_Update.Size = new Size(96, 25);
            this.label_Update.TabIndex = 2;
            this.label_Update.Text = "";
            this.label_Update.Visible = false;
            this.Controls.Add(this.label_Update);

            // label_Game
            this.label_Game.ForeColor = Color.White;
            this.label_Game.AutoSize = true;
            this.label_Game.Location = new Point(12, 84);
            this.label_Game.Name = "label_Game";
            this.label_Game.Size = new Size(96, 25);
            this.label_Game.TabIndex = 3;
            this.label_Game.Text = "Game: loading";
            this.label_Game.Visible = false;
            this.Controls.Add(this.label_Game);

            /*
            // label_Connection
            this.label_Connection.ForeColor = Color.White;
            this.label_Connection.AutoSize = true;
            this.label_Connection.Location = new Point(12, 109);
            this.label_Connection.Name = "label_Connection";
            this.label_Connection.Size = new Size(96, 25);
            this.label_Connection.TabIndex = 4;
            this.label_Connection.Text = "Connection: waiting for game";
            this.label_Connection.Visible = false;
            this.Controls.Add(this.label_Connection);
            */

            // label_Save
            this.label_Save.ForeColor = Color.White;
            this.label_Save.AutoSize = true;
            this.label_Save.Location = new Point(12, /*134*/ 109);
            this.label_Save.Name = "label_Save";
            this.label_Save.Size = new Size(96, 25);
            this.label_Save.TabIndex = /*5*/ 4;
            this.label_Save.Text = "Save: waiting for game";
            this.label_Save.Visible = false;
            this.Controls.Add(this.label_Save);

            // label_Help
            this.label_Help.ForeColor = Color.White;
            this.label_Help.AutoSize = true;
            this.label_Help.Location = new Point(12, /*159*/ 134);
            this.label_Help.Name = "label_Help";
            this.label_Help.Size = new Size(96, 25);
            this.label_Help.TabIndex = /*6*/ 5;
            this.label_Help.Text = "If you need help, you can ask in #setup-support on Discord.";
            this.label_Help.Visible = false;
            this.Controls.Add(this.label_Help);

            // button_Close_Menu
            this.button_Close_Menu.ForeColor = Color.White;
            this.button_Close_Menu.AutoSize = true;
            this.button_Close_Menu.Location = new Point(12, /*184*/ 159);
            this.button_Close_Menu.Name = "button_Close_Menu";
            this.button_Close_Menu.Size = new Size(96, 25);
            this.button_Close_Menu.TabIndex = /*7*/ 6;
            this.button_Close_Menu.Text = "Done";
            this.button_Close_Menu.Visible = false;
            this.button_Close_Menu.Click += new EventHandler((object sender, EventArgs e) => {
                if (this.WindowState == FormWindowState.Maximized) {
                    this.WindowState = FormWindowState.Normal;
                }
                this.FormBorderStyle = FormBorderStyle.FixedSingle;
                this.MaximizeBox = false;
                this.ClientSize = new Size(720, 896);
                this.label_Version.Visible = false;
                this.button_Update.Visible = false;
                this.label_Update.Visible = false;
                this.label_Game.Visible = false;
                //this.label_Connection.Visible = false;
                this.label_Save.Visible = false;
                this.label_Help.Visible = false;
                this.button_Close_Menu.Visible = false;
                foreach (PictureBox cell in this.cells) {
                    cell.Visible = true;
                }
            });
            this.Controls.Add(this.button_Close_Menu);

            ResumeLayout(true);
        }

        /// <summary>
        /// Detect if the current ROM is an OoTMM combo ROM by checking ROM header signatures.
        /// OoTMM ROMs have specific characteristics that distinguish them from standalone OoT/MM ROMs.
        /// </summary>
        /// <returns>True if the ROM appears to be an OoTMM combo ROM</returns>
        private bool DetectComboRomFromHeader() {
            try {
                // Read extended ROM header area to check for OoTMM signatures
                var romHeader = APIs.Memory.ReadByteRange(0x00, 0x50, "ROM");
                var romNameArea = APIs.Memory.ReadByteRange(OoTMMSignatures.ROM_NAME_OFFSET, 0x20, "ROM");

                // Check 1: Look for "OOTMM" signature in ROM name area
                string romName = Encoding.UTF8.GetString(romNameArea.ToArray()).TrimEnd('\0');
                if (romName.Contains("OOTMM") || romName.Contains("OoTMM") || romName.Contains("COMBO")) {
                    this.comboDetectedFromRomHeader = true;
                    return true;
                }

                // Check 2: Check if ROM has both OoT and MM characteristics
                // OoTMM ROMs start with OoT header but have modified characteristics
                var gameId = APIs.Memory.ReadByteRange(OoTMMSignatures.GAME_ID_OFFSET, 4, "ROM");
                string gameIdStr = Encoding.UTF8.GetString(gameId.ToArray());

                // OoTMM may use modified game IDs that aren't standard OoT/MM codes
                bool isStandardOot = gameIdStr.StartsWith("CZL"); // CZLE, CZLJ, CZLP
                bool isStandardMm = gameIdStr.StartsWith("NZS");  // NZSE, NZSJ, NZSP

                // If game ID doesn't match standard patterns, it could be OoTMM
                if (!isStandardOot && !isStandardMm) {
                    // Check for OoTMM-specific modified game IDs
                    if (gameIdStr.Contains("MM") || gameIdStr.Contains("COMBO")) {
                        this.comboDetectedFromRomHeader = true;
                        return true;
                    }
                }

                // Check 3: Verify ROM size indicates combo ROM
                // OoTMM ROMs are significantly larger than standalone ROMs (>64MB)
                // This is a heuristic check - actual implementation may need adjustment
                // Note: ROM size check is platform-dependent and may not work in all emulators

                return false;
            } catch {
                // If ROM reading fails, fall back to memory-based detection
                return false;
            }
        }

        /// <summary>
        /// Detect combo ROM mode using memory context addresses.
        /// OoTMM uses specific memory addresses to track which game is currently active.
        /// </summary>
        /// <returns>True if combo context addresses indicate OoTMM mode</returns>
        private bool DetectComboFromMemoryContext() {
            try {
                // Check OoT combo context address
                var ootContext = APIs.Memory.ReadByteRange(MmAddresses.OOT_COMBO_CONTEXT_ADDR, 4, "RDRAM");
                bool ootContextActive = ootContext.Any(b => b != 0);

                // Check MM combo context address
                var mmContext = APIs.Memory.ReadByteRange(MmAddresses.MM_COMBO_CONTEXT_ADDR, 4, "RDRAM");
                bool mmContextActive = mmContext.Any(b => b != 0);

                // If either context address is active, this is likely a combo ROM
                return ootContextActive || mmContextActive;
            } catch {
                return false;
            }
        }

        /// <summary>
        /// Determine which game (OoT or MM) is currently active in combo mode.
        /// Uses multiple detection strategies for reliability.
        /// </summary>
        /// <returns>The currently active game in combo mode</returns>
        private ComboActiveGame DetectActiveGameInCombo() {
            try {
                // Strategy 1: Check combo context addresses
                var ootContext = APIs.Memory.ReadByteRange(MmAddresses.OOT_COMBO_CONTEXT_ADDR, 4, "RDRAM");
                var mmContext = APIs.Memory.ReadByteRange(MmAddresses.MM_COMBO_CONTEXT_ADDR, 4, "RDRAM");

                bool ootContextActive = ootContext.Any(b => b != 0);
                bool mmContextActive = mmContext.Any(b => b != 0);

                // Clear indication from context addresses
                if (ootContextActive && !mmContextActive) {
                    return ComboActiveGame.OcarinaOfTime;
                }
                if (mmContextActive && !ootContextActive) {
                    return ComboActiveGame.MajorasMask;
                }

                // Strategy 2: Check scene IDs as fallback
                var ootSceneId = APIs.Memory.ReadByte(OoTMMSignatures.SCENE_ID_OOT_ADDR, "RDRAM");

                // Valid OoT scene ID range check
                if (ootSceneId <= OoTMMSignatures.OOT_MAX_SCENE_ID) {
                    // Additional validation: check if this looks like a valid OoT scene
                    // OoT has specific scene patterns
                    return ComboActiveGame.OcarinaOfTime;
                }

                // Strategy 3: Check MM save context for valid data
                var mmSaveHeader = APIs.Memory.ReadByteRange(MmAddresses.MM_SAVE_ADDR, 8, "RDRAM");
                bool mmSaveValid = mmSaveHeader.Any(b => b != 0);

                if (mmSaveValid) {
                    // Check if MM scene data looks valid
                    return ComboActiveGame.MajorasMask;
                }

                // Default to last known state
                return this.comboActiveGame;
            } catch {
                return this.comboActiveGame;
            }
        }

        /// <summary>
        /// Get a human-readable string describing the current combo mode state.
        /// </summary>
        private string GetComboModeStatusString() {
            string activeGameStr = this.comboActiveGame == ComboActiveGame.OcarinaOfTime ? "OoT" : "MM";
            string detectionMethod = this.comboDetectedFromRomHeader ? "ROM header" : "memory context";
            return $"Playing OoTMM combo ({activeGameStr} active, detected via {detectionMethod})";
        }

        public override void Restart() {
            if (!this.initialized) {
                using (var cfg_res = Native.config_load()) {
                    if (cfg_res.IsOk()) {
                        if (!cfg_res.IsOkSome()) {
                            this.DialogController.ShowMessageBox(this, "Welcome to the OoT auto-tracker!\nTo change settings, right-click a Medallion.");
                        }
                        this.cfg = cfg_res.UnwrapUnwrapOrDefault();
                        UpdateCells();
                        if (!cfg.UpdateCheckIsSome()) {
                            using (var res = this.cfg.SetUpdateCheck(this.DialogController.ShowMessageBox2(this, "Check for updates on startup?"))) {
                                if (!res.IsOk()) {
                                    this.DialogController.ShowMessageBox(this, $"failed to save config file: {res.DebugErr().ToString()}");
                                }
                            }
                        }
                        if (this.cfg.UpdateCheck()) {
                            CheckForUpdates();
                        }
                    } else {
                        this.DialogController.ShowMessageBox(this, $"failed to load config file: {cfg_res.DebugErr().ToString()}");
                    }
                }
                this.initialized = true;
            }

            APIs.Memory.SetBigEndian(true);
            this.model.Dispose();
            this.detectedGame = GameType.Unknown;
            this.prevMmSaveData = null;
            // Reset combo mode state
            this.isComboRom = false;
            this.comboActiveGame = ComboActiveGame.OcarinaOfTime;
            this.comboDetectedFromRomHeader = false;
            this.comboContextCheckFrames = 0;
            /*
            if (this.stream != null) { this.stream.Disconnect().Dispose(); }
            this.stream = null;
            UpdateConnection(false, "Connection: waiting for game");
            */
            if (this.prevSave != null) { this.prevSave.Dispose(); }
            this.prevSave = null;
            UpdateSave(false, "Save: waiting for game");
            if ((APIs.GameInfo.GetGameInfo()?.Name ?? "Null") == "Null") {
                this.model = ModelState.FromSaveAndKnowledge(Native.save_default(), Native.knowledge_none());
                UpdateGame(false, "Not playing anything");
            } else {
                var rom_ident = APIs.Memory.ReadByteRange(0x20, 0x18, "ROM");
                // Check for OoT ROM: "THE LEGEND OF ZELDA \0"
                bool isOotRom = Enumerable.SequenceEqual(rom_ident.GetRange(0, 0x15), new List<byte>(Encoding.UTF8.GetBytes("THE LEGEND OF ZELDA \0")));
                // Check for MM ROM: "ZELDA MAJORA'S MASK " at offset 0x20
                bool isMmRom = Enumerable.SequenceEqual(rom_ident.GetRange(0, 0x14), new List<byte>(Encoding.UTF8.GetBytes("ZELDA MAJORA'S MASK ")));

                // Priority 1: Check for OoTMM combo ROM via ROM header signature
                bool isComboFromHeader = DetectComboRomFromHeader();

                if (isComboFromHeader) {
                    // OoTMM combo ROM detected from ROM header
                    this.isComboRom = true;
                    this.detectedGame = GameType.Combo;
                    this.comboActiveGame = DetectActiveGameInCombo();
                    this.model = ModelState.FromSaveAndKnowledge(Native.save_default(), Native.knowledge_none());
                    UpdateGame(true, GetComboModeStatusString());
                } else if (!isOotRom && !isMmRom) {
                    // Unknown ROM - could still be OoTMM with non-standard header
                    // Try memory-based detection as fallback
                    bool isComboFromMemory = DetectComboFromMemoryContext();
                    if (isComboFromMemory) {
                        this.isComboRom = true;
                        this.detectedGame = GameType.Combo;
                        this.comboActiveGame = DetectActiveGameInCombo();
                        this.model = ModelState.FromSaveAndKnowledge(Native.save_default(), Native.knowledge_none());
                        UpdateGame(true, GetComboModeStatusString());
                    } else {
                        this.model = ModelState.FromSaveAndKnowledge(Native.save_default(), Native.knowledge_none());
                        UpdateGame(false, $"Game: Expected OoT/OoTR/MM/MMR/OoTMM, found {APIs.GameInfo.GetGameInfo()?.Name ?? "Null"} ({string.Join<byte>(", ", rom_ident.GetRange(0, 0x15))})");
                    }
                } else if (isMmRom) {
                    // Majora's Mask detected
                    this.detectedGame = GameType.MajorasMask;
                    var version = rom_ident.GetRange(0x14, 4);
                    this.isVanilla = Enumerable.SequenceEqual(version, new List<byte>(new byte[] { 0, 0, 0, 0 }));
                    this.model = ModelState.FromSaveAndKnowledge(Native.save_default(), Native.knowledge_none());
                    if (this.isVanilla) {
                        UpdateGame(true, "Playing MM (vanilla)");
                    } else {
                        UpdateGame(true, $"Playing MM randomizer");
                    }
                } else {
                    // OoT ROM detected - check if it's actually a combo ROM via memory context
                    var version = rom_ident.GetRange(0x15, 3);
                    this.isVanilla = Enumerable.SequenceEqual(version, new List<byte>(new byte[] { 0, 0, 0 }));

                    // Check for combo randomizer using multiple detection methods
                    bool isComboFromMemory = DetectComboFromMemoryContext();

                    if (isComboFromMemory) {
                        // OoTMM combo ROM detected from memory context
                        this.isComboRom = true;
                        this.detectedGame = GameType.Combo;
                        this.comboActiveGame = DetectActiveGameInCombo();
                        this.model = ModelState.FromSaveAndKnowledge(Native.save_default(), Native.knowledge_none());
                        UpdateGame(true, GetComboModeStatusString());
                    } else {
                        // Standard OoT/OoTR ROM
                        this.detectedGame = GameType.OcarinaOfTime;
                        this.model = ModelState.FromSaveAndKnowledge(Native.save_default(), this.isVanilla ? Native.knowledge_vanilla() : Native.knowledge_none());
                        if (this.isVanilla) {
                            UpdateGame(true, "Playing OoT (vanilla)");
                        } else {
                            UpdateGame(true, $"Playing OoTR version {version[0]}.{version[1]}.{version[2]}");
                        }
                    }
                    /*
                    using (var stream_res = TcpStreamResult.Connect(IPAddress.IPv6Loopback)) { //TODO only connect manually
                        if (stream_res.IsOk()) {
                            if (this.stream != null) { this.stream.Disconnect().Dispose(); }
                            this.stream = stream_res.Unwrap();
                            UpdateConnection(true, "Connected");
                            if (this.isVanilla) {
                                using (var knowledge = Native.knowledge_vanilla()) { //TODO pull knowledge back out of this.model
                                    knowledge.Send(this.stream);
                                }
                            }
                        } else {
                            using (StringHandle err = stream_res.DebugErr()) {
                                UpdateConnection(false, $"Failed to connect: {err.AsString()}");
                            }
                        }
                    }
                    */
                }
            }
            UpdateCells();
        }

        public override void UpdateValues(ToolFormUpdateType type) {
            if (type != ToolFormUpdateType.PreFrame) { return; } //TODO setting to also enable auto-tracking during turbo (ToolFormUpdateType.FastPreFrame)?
            if ((APIs.GameInfo.GetGameInfo()?.Name ?? "Null") == "Null") { return; }

            // For combo mode: periodically re-check which game is active
            if (this.isComboRom && this.detectedGame == GameType.Combo) {
                this.comboContextCheckFrames++;
                if (this.comboContextCheckFrames >= COMBO_CONTEXT_CHECK_INTERVAL) {
                    this.comboContextCheckFrames = 0;
                    var previousActiveGame = this.comboActiveGame;
                    this.comboActiveGame = DetectActiveGameInCombo();

                    // Update status if active game changed
                    if (previousActiveGame != this.comboActiveGame) {
                        UpdateGame(true, GetComboModeStatusString());
                        // Reset RAM state when switching games to ensure clean reads
                        this.rawRam = null;
                        this.prevMmSaveData = null;
                    }
                }
            }

            // Handle MM save context reading
            // In combo mode, only read MM data when MM is the active game
            bool shouldReadMm = this.detectedGame == GameType.MajorasMask ||
                (this.detectedGame == GameType.Combo && this.comboActiveGame == ComboActiveGame.MajorasMask);
            if (shouldReadMm) {
                ReadMmSaveContext();
            }

            // Handle OoT/combo save context reading
            // In combo mode, only read OoT data when OoT is the active game
            bool shouldReadOot = this.detectedGame == GameType.OcarinaOfTime ||
                (this.detectedGame == GameType.Combo && this.comboActiveGame == ComboActiveGame.OcarinaOfTime);
            if (shouldReadOot) {
                if (this.autoTrackerContextAddr == null && Enumerable.SequenceEqual(APIs.Memory.ReadByteRange(0x11a5d0 + 0x1c, 6, "RDRAM"), new List<byte>(Encoding.UTF8.GetBytes("ZELDAZ")))) { // don't check auto-tracker context version while rom is loaded but not properly initialized
                    var randoContextAddr = 0x8040_0000;
                    var newAutoTrackerContextAddr = APIs.Memory.ReadU32(randoContextAddr + 0xc, "System Bus");
                    if (newAutoTrackerContextAddr >= 0x8000_0000 && newAutoTrackerContextAddr != 0xffff_ffff) {
                        this.autoTrackerContextAddr = newAutoTrackerContextAddr;
                        this.autoTrackerContextVersion = APIs.Memory.ReadU32(newAutoTrackerContextAddr, "System Bus");
                        var length = 0;
                        switch (this.autoTrackerContextVersion) {
                            case 0: {
                                // no extra features supported
                                break;
                            }
                            case 1: {
                                length = 0x38;
                                break;
                            }
                            default: {
                                throw new NotImplementedException($"auto-tracker context version {this.autoTrackerContextVersion} not supported"); //TODO display error instead of crashing
                            }
                        }
                        if (length > 0) {
                            this.model.SetAutoTrackerContext(APIs.Memory, newAutoTrackerContextAddr, length);
                        }
                    }
                }
                bool changed = true;
                if (this.rawRam == null) {
                    this.rawRam = new RawRam(APIs.Memory);
                } else {
                    changed = this.rawRam.Update(APIs.Memory);
                }
                if (!changed) { return; }
                using (var ram_res = this.rawRam.ToRam()) {
                    if (ram_res.IsOk()) {
                        var ram = ram_res.Unwrap();
                        if (prevRam != null && ram.Equals(prevRam)) { return; }
                        if (prevRam != null) { prevRam.Dispose(); }
                        prevRam = ram;
                    } else {
                        UpdateSave(false, $"Failed to read game RAM: {ram_res.DebugErr().AsString()}");
                        return;
                    }
                }
                UpdateSave(true, $"Save data ok, last checked {DateTime.Now}");
                this.model.SetRam(prevRam);
                UpdateCells();
                var save = prevRam.CloneSave();
                if (prevSave != null && save.Equals(prevSave)) { return; }
                if (prevSave == null) {
                    /*
                    if (this.stream != null) {
                        using (UnitResult unit_res = save.Send(this.stream)) {
                            if (!unit_res.IsOk()) {
                                if (this.stream != null) { this.stream.Dispose(); }
                                this.stream = null;
                                using (StringHandle err = unit_res.DebugErr()) {
                                    UpdateConnection(false, $"Failed to send save data: {err.AsString()}");
                                }
                            } else {
                                UpdateConnection(true, $"Connected, initial save data sent {DateTime.Now}");
                            }
                        }
                    }
                    */
                    prevSave = save;
                } else if (!save.Equals(prevSave)) {
                    /*
                    if (this.stream != null) {
                        using (SavesDiff diff = prevSave.Diff(save)) {
                            using (UnitResult unit_res = diff.Send(this.stream)) {
                                if (!unit_res.IsOk()) {
                                    if (this.stream != null) { this.stream.Dispose(); }
                                    this.stream = null;
                                    using (StringHandle err = unit_res.DebugErr()) {
                                        UpdateConnection(false, $"Failed to send save data: {err.AsString()}");
                                    }
                                } else {
                                    UpdateConnection(true, $"Connected, save data last sent {DateTime.Now}");
                                }
                            }
                        }
                    }
                    */
                    prevSave.Dispose();
                    prevSave = save;
                } else {
                    save.Dispose();
                }
            }
        }

        /// <summary>
        /// Read MM save context from RDRAM
        /// </summary>
        private void ReadMmSaveContext() {
            try {
                // Read MM save context data
                var mmSaveData = APIs.Memory.ReadByteRange(MmAddresses.MM_SAVE_ADDR, MmAddresses.MM_SAVE_SIZE, "RDRAM").ToArray();

                // Check if data has changed
                if (this.prevMmSaveData != null && Enumerable.SequenceEqual(mmSaveData, this.prevMmSaveData)) {
                    return;
                }

                this.prevMmSaveData = mmSaveData;

                // Update status message based on mode
                string modePrefix = this.isComboRom ? "OoTMM (MM mode)" : "MM";
                UpdateSave(true, $"{modePrefix} save data ok, last checked {DateTime.Now}");

                // TODO: Once MM save parsing is implemented in Rust FFI, process the data here
                // For now, we just read and store the raw bytes for future use
            } catch (Exception ex) {
                // Handle read errors gracefully, especially during game transitions in combo mode
                if (this.isComboRom) {
                    // In combo mode, read failures during game transitions are expected
                    // Don't update the status to avoid flickering
                } else {
                    UpdateSave(false, $"Failed to read MM save data: {ex.Message}");
                }
            }
        }

        private void UpdateCells() {
            using (var layout = this.cfg.Layout()) {
                for (byte i = 0; i < 52; i++) {
                    using (TrackerCell cell = layout.Cell(i)) {
                        string new_img = cell.Image(this.model).AsString();
                        if (new_img == this.cellImages[i]) { continue; }
                        this.cellImages[i] = new_img;
                        var stream = typeof(MainForm).Assembly.GetManifestResourceStream($"Net.Fenhl.OotAutoTracker.Resources.{new_img}.png");
                        if (stream == null) { throw new Exception($"image stream for cell {i} ({new_img}) is null"); }
                        this.cells[i].Image = Image.FromStream(stream);
                    }
                }
            }
        }

        private void UpdateGame(bool ok, String msg) {
            label_Game.Text = msg;
            this.gameOk = ok;
            UpdateHelpLabel();
        }

        /*
        private void UpdateConnection(bool ok, String msg) {
            label_Connection.Text = msg;
            this.connectionOk = ok;
            UpdateHelpLabel();
        }
        */

        private void UpdateSave(bool ok, String msg) {
            label_Save.Text = msg;
            this.saveOk = ok;
            UpdateHelpLabel();
        }

        private void UpdateHelpLabel() {
            if (this.gameOk /*&& this.connectionOk*/ && this.saveOk) {
                label_Help.Text = "";
            } else {
                label_Help.Text = "If you need help, you can ask in #setup-support on Discord.";
            }
        }

        private void CheckForUpdates() {
            this.label_Update.Text = "Checking for updates…";
            using (var update_available_res = Native.update_available()) {
                if (update_available_res.IsOk()) {
                    if (update_available_res.Unwrap()) {
                        this.label_Update.Text = "An update is available";
                        using (var run_updater_res = Native.run_updater()) {
                            if (!run_updater_res.IsOk()) {
                                this.label_Update.Text = run_updater_res.DebugErr().AsString();
                            }
                        }
                    } else {
                        this.label_Update.Text = $"You are up to date as of {DateTime.Now}";
                    }
                } else {
                    this.label_Update.Text = update_available_res.DebugErr().AsString();
                }
            }
        }
    }
}
