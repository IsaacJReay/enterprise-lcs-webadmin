import React from "react";
import { Modal, Button, Form, Input, Select, Checkbox } from "antd";
import { FiX } from "react-icons/fi";

const { Option } = Select;

const CreateDomain = ({ handleCancel, handleOk, visible }) => {
  const layout = {
    labelCol: {
      span: 8,
    },
    wrapperCol: {
      span: 16,
    },
  };

  return (
    <React.Fragment>
      <Modal
        width={800}
        title="Creating Domain"
        footer={null}
        closeIcon={<FiX className="close-icon" />}
        visible={visible}
        onCancel={handleCancel}
        onOk={handleOk}
      >
        <div className="container-adding-records">
          <Form {...layout} size="large">
            <Form.Item
              label="Domain name"
              name="name"
              rules={[{ required: true, message: "Please input domain name!" }]}
            >
              <Input placeholder="text here ..." size="large" />
            </Form.Item>
            <Form.Item label="Status" name="status">
              <Checkbox>Hosting</Checkbox>
            </Form.Item>
            <Form.Item>
              <Button
                className="button-apply"
                size="large"
                htmlType="button"
                type="primary"
              >
                Submit
              </Button>
            </Form.Item>
          </Form>
        </div>
      </Modal>
    </React.Fragment>
  );
};

export default CreateDomain;
