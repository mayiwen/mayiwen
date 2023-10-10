import {
  AfterViewInit,
  ChangeDetectorRef,
  Component,
  EventEmitter,
  Input,
  OnInit,
  Output,
  ViewChild,
} from '@angular/core';
import { title } from 'process';
import {
  createEditor,
  createToolbar,
  IToolbarConfig,
} from '@wangeditor/editor';
import { MindMapService } from '../../mind-map.service';

@Component({
  selector: 'app-wangeditor',
  templateUrl: './wangeditor.component.html',
  styleUrls: ['./wangeditor.component.less'],
  providers: [],
})
export class WangEditorComponent implements OnInit, AfterViewInit {
  @Input() id = '';
  @Output('changeHtml') changeHtml = new EventEmitter();
  private editor: any; // 绑定的文本内容
  constructor(private ms: MindMapService) {}
  ngAfterViewInit(): void {
    let _changeHtml = this.changeHtml;
    let _this = this;
    console.log('这是afterViewInit');
    console.log(this.id);
    const editorConfig = {
      placeholder: '请在这里输入内容',
      // autoFocus: false, // 配置编辑器默认是否 focus ，默认为 true
      MENU_CONF: {
        uploadImage: {
          fieldName: 'your-fileName',
          base64LimitSize: 10 * 1024 * 1024 * 1024,
        },
        codeSelectLang: {
          codeLangs: [
            { text: 'Typescript', value: 'typescript' },
            { text: 'HTML', value: 'html' },
            { text: 'CSS', value: 'css' },
            { text: 'SQL', value: 'sql' },
            { text: 'Markdown', value: 'markdown' },
            { text: 'XML', value: 'xml' },
            { text: 'Javascript', value: 'javascript' },
            { text: 'JSX', value: 'jsx' },
            { text: 'Go', value: 'go' },
            { text: 'PHP', value: 'php' },
            { text: 'C', value: 'c' },
            { text: 'Python', value: 'python' },
            { text: 'Java', value: 'java' },
            { text: 'C++', value: 'cpp' },
            { text: 'C#', value: 'csharp' },
            { text: 'Visual Basic', value: 'visual-basic' },
            { text: 'Ruby', value: 'ruby' },
            { text: 'Swift', value: 'swift' },
            { text: 'Bash', value: 'bash' },
            { text: 'Lua', value: 'lua' },
            { text: 'Groovy', value: 'groovy' },
          ],
        },
        lineHeight: {
          lineHeightList: ['0.4', '0.5', '1', '1.5', '2', '2.5']
        },
        fontFamily: {
          fontFamilyList: [
            { name: 'myw001', value: 'myw001' },
            '黑体',
            '楷体',
            { name: '仿宋', value: '仿宋' },
            'Arial',
            'Tahoma',
            'Verdana',
          ]
        }
      },
      content: [
        {
          type: 'paragraph',
          children: [
            { text: '', fontFamily: 'myw001, 微软雅黑', fontSize: '16px' },
          ],
          lineHeight: 0.8,
        },
      ],
      onChange(edit: any) {
        const html = edit.getHtml();
        if (_this.ms.flagKillLoop) {
          _changeHtml.emit(html);
        }
      },
      onchangeTimeout: 1000,
      customUploadImg: (resultFiles: any, insertImgFn: any) => {
        console.log('调用了自定义图片上传事件');
        /**
         * resultFiles：图片上传文件流
         * insertImgFn：插入图片到富文本
         * 返回结果为生成的图片URL地址
         *  */
        // 此方法为自己封赚改写的阿里云图片OSS直传插件。
        // this.$oss(resultFiles[0], resultFiles[0]["name"]).then(url => {
        // 	!!url && insertImgFn(url); /* oss图片上传，将图片插入到编辑器中 */
        // });
      },
    };

    const toolbarConfig: Partial<IToolbarConfig> = {
      // TS 语法
      // const toolbarConfig = {
      toolbarKeys: [
        'codeBlock',
        'headerSelect',
        {
          key: 'group-more-style',
          title: '更多',
          iconSvg: `<svg viewBox="0 0 1024 1024"><path d="M204.8 505.6m-76.8 0a76.8 76.8 0 1 0 153.6 0 76.8 76.8 0 1 0-153.6 0Z"></path><path d="M505.6 505.6m-76.8 0a76.8 76.8 0 1 0 153.6 0 76.8 76.8 0 1 0-153.6 0Z"></path><path d="M806.4 505.6m-76.8 0a76.8 76.8 0 1 0 153.6 0 76.8 76.8 0 1 0-153.6 0Z"></path></svg>`,
          menuKeys: [
            'bold',
            'underline',
            'italic',
            'through',
            'code',
            'sup',
            'sub',
            'clearStyle',
            'blockquote',
          ],
        },
        {
          key: 'group-more-style11',
          title: '更多',
          iconSvg: `<svg viewBox="0 0 1024 1024"><path d="M204.8 505.6m-76.8 0a76.8 76.8 0 1 0 153.6 0 76.8 76.8 0 1 0-153.6 0Z"></path><path d="M505.6 505.6m-76.8 0a76.8 76.8 0 1 0 153.6 0 76.8 76.8 0 1 0-153.6 0Z"></path><path d="M806.4 505.6m-76.8 0a76.8 76.8 0 1 0 153.6 0 76.8 76.8 0 1 0-153.6 0Z"></path></svg>`,
          menuKeys: [
            'color',
            'bgColor',
            'fontSize',
            'fontFamily',
            'lineHeight',
            'bulletedList',
            'numberedList',
            'todo',
          ],
        },

        {
          key: 'group-justify',
          title: '对齐',
          iconSvg: `<svg viewBox="0 0 1024 1024"><path d="M768 793.6v102.4H51.2v-102.4h716.8z m204.8-230.4v102.4H51.2v-102.4h921.6z m-204.8-230.4v102.4H51.2v-102.4h716.8zM972.8 102.4v102.4H51.2V102.4h921.6z"></path></svg>`,
          menuKeys: [
            'justifyLeft',
            'justifyRight',
            'justifyCenter',
            'justifyJustify',
            'indent',
            'delIndent',
          ],
        },
        {
          key: 'group-indent',
          title: '缩进',
          iconSvg: `<svg viewBox="0 0 1024 1024"><path d="M0 64h1024v128H0z m384 192h640v128H384z m0 192h640v128H384z m0 192h640v128H384zM0 832h1024v128H0z m0-128V320l256 192z"></path></svg>`,
          menuKeys: ['emotion', 'insertLink', 'insertVideo', 'divider'],
        },

        'insertTable',
        '|',
        'undo',
        'redo',
        '|',
        'fullScreen',
      ],
      excludeKeys: [],
      insertKeys: { index: 0, keys: [] },
      modalAppendToBody: false,
      /* 工具栏配置 */
    };
    // 自定义上传图片
    const editor = createEditor({
      selector: `#${this.id}-editor-container`,
      html: `<p><br></p>`,
      config: editorConfig,
      mode: 'simple',
    });
    const toolbar = createToolbar({
      editor,
      selector: `#${this.id}-toolbar-container`,
      mode: 'simple',
      config: toolbarConfig,
    });
    toolbar;
    this.editor = editor;
    console.log(this.editor);
  }
  ngOnInit(): void {
    // throw new Error('Method not implemented.');
    console.log('oninit');
  }
  getEditorHtml(): string {
    return this.editor.getHtml();
  }
  setEditorHtml(text: string) {
    console.log('设置了text')
    console.log(text)
    this.editor.select([0, 0])
    this.editor.setHtml(text.trim());
  }
  clear() {
    this.ms.flagKillLoop = false;
    this.editor.clear();
    this.ms.flagKillLoop = true;
  }
}
